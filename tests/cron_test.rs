
/// 定时任务
#[cfg(test)]
mod cron_test{
    use std::str::FromStr;
    use cron::Schedule;
    use chrono::Utc;

    #[test]
    fn test_cron(){
        //                                       每个月的 1日或者15日 5月->8月  每周
        //                  second  min   hour   day of month   month   day of week   year (从 2018起，间隔2年)
        //let expression = "0   30   9,12,15     1,15       May-Aug  Mon,Wed,Fri  2018/2";
        let expression = "*   30   9,12,15 *  *  * *";// (* 表示今年)
        // 初始化一个调度器
        let schedule = Schedule::from_str(expression).unwrap();
        println!("定时任务开启");
        for datetime in schedule.upcoming(Utc).take(100){
            println!("-> {}", datetime);
        }
    }

}