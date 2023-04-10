use super::ItemEnc;
use super::Scalar;
use super::util::*;
use super::entries::*;

/*
T is the type of the data to associate with each vector.
 */
pub struct VectorDb<IT> {
    pub v_len: usize,
    data:Vec<ItemEncOpt<IT>>,
}
impl<IT> VectorDb<IT> {
    pub fn new(len:usize) -> Self {
        return VectorDb { v_len: len, data: Vec::new() };
    }

    pub fn insert_all(&mut self, data:Vec<ItemEnc<IT>>) {
        // sort/partition it by shard and then send blocks to different nodes (threads in local case)
        // self.partition(&mut data);
        // let itr = data.iter()
        //         .map(|item| ItemEncOpt { v: VecData::new(item.v), val: item.val });
        for item in data {
            let item = ItemEncOpt { v: VecData::new(item.v), val: item.val };
            self.data.push(item);
        }
    }

    pub fn query(&mut self, v1: Vec<Scalar>, limit: usize) -> Vec<(&IT, &VecData, Scalar)> {
        return self.query_brute(v1, limit);
    }

    pub fn query_brute(&mut self, query: Vec<Scalar>, limit: usize) -> Vec<(&IT, &VecData, Scalar)> {
        let query2 = VecData::new(query);
        let mut similarities: Vec<(usize,Scalar)> = self.data.iter()
                .map(|it| similarity(&it.v, &query2)).enumerate().collect();
        similarities.sort_unstable_by(|(_, x1), (_, x2)| x2.partial_cmp(x1).unwrap());
        return similarities.iter().take(limit).map(|x| self.to_ret(*x)).collect();
        // |(i,sim)| (self.data[i]
    }

    fn to_ret(&self, (i, sim): (usize, Scalar)) -> (&IT, &VecData, Scalar) {
        let item = &self.data[i];
        return (&item.val, &item.v, sim);
    }

    // For sharding, will simulate by using multiple threads
    // fn partition(&self, data:&mut Data) {
    //     // self has partition info
    // }
}