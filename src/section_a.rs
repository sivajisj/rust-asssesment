fn max_maintenance_reqs(
mut intervals: Vec<[i32;2]>)
    
->i32{
    intervals.sort_by_key(|x| x[1]);
    
    //intializing count , end time
    
    let mut count = 0; //intial value: 0
    let mut last_end_time = i32::MIN; //for tracking last picked tasjks ending
    
    for task in intervals{
        let start = task[0];
        let end = task[1];
        
        if start >= last_end_time{
            count += 1;
            last_end_time = end // for update timeline
        }
    }
    
     count
    
}

//main function

// - *Input*: `[[900, 1030], [1000, 1100], [1030, 1130], [1100, 1200]]`

fn  main(){
    let tasks = vec![
    [900, 1030],
    [1000, 1100],
    [1030, 1130],
    [1100, 1200]
    ];
    
    let  result = max_maintenance_reqs(tasks);
    println!("max non overlapping requests : {}", result);
}
