use super::*;
use super::util::*;
use super::entries::*;

/**
 *
 * How about an online principal components algorithm?
 * or the space filling curve + intmap approach?
 * or project onto various vectors, binary search, merge neighbors for each and sort those?
 *
 * probably vectors don't matter as long as they cover, so can optimize by just
 * shopping up the vectors into n pieces
 */


struct IndexEntry {
    proj: Scalar,
    item_ind: usize,
}

/*
T is the type of the data to associate with each vector.
 */
pub struct VectorDb<IT:Debug> {
    pub v_len: usize,
    // projections:Vec<VKey>,
    data:Vec<ItemEncOpt<IT>>,
    index:Vec<Vec<IndexEntry>>,
}
impl<IT:Debug> VectorDb<IT> {
    pub fn new(len:usize) -> Self {
        let proj_count = 100.0; //(len as f32).log2().ceil();
        let pc = proj_count as usize;
        let mut index: Vec<Vec<IndexEntry>> = (0..pc).map(|_| Vec::new()).collect();
        for i in 0..pc {
            index[i] = Vec::new();
        }
        return VectorDb { v_len: len, data: Vec::new(), index };
    }

    pub fn insert(&mut self, item: ItemEnc<IT>) {
        let item_ind = self.data.len();
        let projs = proj_itr(&item.v, self.index.len());
        for (ind_index, proj) in projs.iter().enumerate() {
            let index_ind = &mut self.index[ind_index];
            index_ind.push(IndexEntry { proj: *proj, item_ind });
            index_ind.sort_unstable_by(|item1, item2|
                    item2.proj.partial_cmp(&item1.proj).unwrap());
            index_ind.truncate(1000);
        }

        let v = VecData::new(item.v);
        let item = ItemEncOpt { v, val: item.val };
        self.data.push(item);
    }

    pub fn insert_all(&mut self, data:Vec<ItemEnc<IT>>) {
        for item in data {
            self.insert(item);
        }
    }

    pub fn query_brute(&self, query: &VKey, limit: usize) -> Vec<Result<IT>> {
        let query2 = VecData::new(query.clone());
        let mut similarities: Vec<(usize,Scalar)> = self.data.iter()
                .map(|it| similarity(&it.v, &query2)).enumerate().collect();
        similarities.sort_unstable_by(|(_, x1), (_, x2)| x2.partial_cmp(x1).unwrap());
        return similarities.iter().take(limit).map(|x| self.to_ret(*x)).collect();
    }

    pub fn query(&self, query: &VKey, limit: usize) -> Vec<Result<IT>> {
        const SPREAD: usize = 100;
        let mut cand_inds: Vec<usize> = Vec::new();
        let projs = proj_itr(query, self.index.len());
        for (ind_index, proj) in projs.iter().enumerate() {
            let index_ind = &self.index[ind_index];
            let res = index_ind.binary_search_by(
                    |probe| probe.proj.partial_cmp(&proj).unwrap());
            match res {
                Ok(ind) | Err(ind) => {
                    let range = range_around(ind, SPREAD, index_ind.len());
                    cand_inds.extend(range.map(|i| index_ind[i].item_ind));
                }
            }
        }
        // TODO: optimize
        cand_inds.sort();
        cand_inds.dedup();
        let mut cands: Vec<Result<IT>> = cand_inds.iter().map(|i| {
            let c = &self.data[*i];
            let sim = dot_norm(query, &c.v.v);
            return Result { sim, val: &c.val, key: &c.v.v };
        }).collect();

        cands.sort_unstable_by(|cand1, cand2| cand2.sim.partial_cmp(&cand1.sim).unwrap());
        cands.truncate(limit);
        return cands;
    }

    fn to_ret(&self, (i, sim): (usize, Scalar)) -> Result<IT> {
        let item = &self.data[i];
        return Result { sim, val: &item.val, key: &item.v.key() };
    }

    // For sharding, will simulate by using multiple threads
    // fn partition(&self, data:&mut Data) {
    //     // self has partition info
    // }
}