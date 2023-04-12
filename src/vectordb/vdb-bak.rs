use super::*;
use super::util::*;
use super::entries::*;

/**
 *
 * How about an online principal components algorithm?
 * or the space filling curve + intmap approach?
 * or project onto various vectors, binary search, merge neighbors for each and sort those?
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
    projections:Vec<VKey>,
    data:Vec<ItemEncOpt<IT>>,
    index:Vec<Vec<IndexEntry>>,
}
impl<IT:Debug> VectorDb<IT> {
    pub fn new(len:usize) -> Self {
        let proj_count = 100.0; //(len as f32).log2().ceil();
        let pc = proj_count as usize;
        let mut projections = vec![Vec::new(); pc]; // Vec::with_capacity(pc);
        let mut index: Vec<Vec<IndexEntry>> = (0..pc).map(|_| Vec::new()).collect();
        for i in 0..pc {
            let mut proj = vec![1.0/proj_count; len];
            for j in (i..pc).step_by(pc) {
                proj[j] = 1.0;
            }
            normalize(&mut proj);
            projections[i] = proj;
            index[i] = Vec::new();
        }
        return VectorDb { v_len: len, projections, data: Vec::new(), index };
    }

    // fn project<'a>(&'a self, v:&'a VKey) -> impl Iterator<Item = (usize,Scalar)> + 'a {
    //     self.projections.iter().map(|p| dot(p, v)).enumerate()
    // }

    pub fn insert(&mut self, item: ItemEnc<IT>) {
        let item_ind = self.data.len();
        // let itr = self.project(&item.v);
        let itr = project(&self.projections, &item.v);
        // println!("Inserting {:?}", item.val);
        for (i, proj) in itr {
            let index_i = &mut self.index[i];
            index_i.push(IndexEntry { proj, item_ind });
            index_i.sort_unstable_by(|item1, item2| item2.proj.partial_cmp(&item1.proj).unwrap());
            // println!("{i} {:?}", index_i[0].proj);
            index_i.truncate(100);
        }
        // for i in 0..self.projections.len() {
        //     let proj = dot(&self.projections[i], &item.v);
        //     self.index[i].push(IndexEntry { proj, item_ind });
        // }

        let v = VecData::new(item.v);
        let item = ItemEncOpt { v, val: item.val };
        self.data.push(item);
    }

    pub fn insert_all(&mut self, data:Vec<ItemEnc<IT>>) {
        // sort/partition it by shard and then send blocks to different nodes (threads in local case)
        // self.partition(&mut data);
        // let itr = data.iter()
        //         .map(|item| ItemEncOpt { v: VecData::new(item.v), val: item.val });
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
        let query_mag = calc_mag(query);
        let mut cands: Vec<Result<IT>> = Vec::new();

        let itr = project(&self.projections, &query);
        for (i, proj) in itr {
            let index_i = &self.index[i];
            let res = index_i.binary_search_by(
                    |probe| probe.proj.partial_cmp(&proj).unwrap());
            match res {
                Ok(ind) | Err(ind) => {
                    let range = range_around(ind, SPREAD, index_i.len());
                    // println!("{i} {:?}", (&range, ind));
                    let itr = range.map(
                            |index_cand_i| {
                                let cand_entry = &index_i[index_cand_i];
                                let data_i = cand_entry.item_ind;
                                let cand = &self.data[data_i];
                                let sim = similarity2(&cand.v, query, query_mag);
                                // return Result { sim, val: &cand.val, key: &cand.v.v };
                                return self.to_ret((data_i, sim));
                            });
                    cands.extend(itr);
                }
            }
        }
        cands.sort_unstable_by(|cand1, cand2| cand2.sim.partial_cmp(&cand1.sim).unwrap());
        // println!("after sort {:?}", cands);
        cands.dedup_by(|cand1, cand2| cand1.sim == cand2.sim);
        // println!("before truncate {}", cands.len());
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