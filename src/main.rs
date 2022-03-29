use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::vec::Vec;

// T是不安全的引用类型则无法在多线程下计算，所以必须加上Send, D同理.
fn worksplit<T: Send + 'static, D: 'static + Send>(
    mut input: Vec<T>,
    work_func: fn(d: Vec<T>) -> Vec<D>,
    split_size: usize,
) -> Vec<D> {
    let res = Arc::new(Mutex::new(Vec::<D>::new()));

    let mut input_len = input.len();
    let mut compute_count = input_len / split_size + 1;
    let mut ths = Vec::new();
    while (compute_count > 0) {
        let mut vs2 = Vec::new();
        let mut split_size = split_size;
        while (split_size > 0 && input_len > 0) {
            vs2.push(input.pop().unwrap());
            split_size -= 1;
            input_len -= 1;
        }
        let res = Arc::clone(&res);
        ths.push(thread::spawn(move || {
            let mut work_output = work_func(vs2);
            for it in work_output.into_iter() {
                res.lock().unwrap().push(it);
            }
        }));
        compute_count -= 1;
    }

    for it in ths {
        it.join();
    }

    let mut output = Vec::<D>::new();
    let mut res_len = res.lock().unwrap().len();
    while (res_len > 0) {
        let v = res.lock().unwrap().pop();
        output.push(v.unwrap());
        res_len -= 1;
    }
    return output;
}

fn main() {
    let mut input = vec![1, 2, 3, 4, 5, 6, 7];
    println!("input: {:?}", input);

    let work_func: fn(d: Vec<_>) -> Vec<_> = |d: Vec<i32>| -> Vec<String> {
        let mut rstrs = Vec::<String>::new();
        for it in d.iter() {
            rstrs.push(it.to_string());
        }
        return rstrs;
    };
    let split_size = 3usize;

    let output = worksplit(input, work_func, split_size);

    println!("output: {:?}", output);
}
