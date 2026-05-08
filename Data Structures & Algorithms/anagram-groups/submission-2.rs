use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut groups:HashMap<[u8;26],Vec<String>>=HashMap::new();

        for word in strs{
            let mut count=[0;26];
            for ch in word.as_bytes(){
                count[(ch-b'a') as usize]+=1;
            }
            groups.entry(count).or_insert(Vec::new()).push(word);
        }
        groups.into_values().collect()
    }
}