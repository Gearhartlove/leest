// problem: https://leetcode.com/problems/baseball-game/

pub fn cal_points(ops: Vec<&str>) -> i32 {
    let mut mut_ops = ops.clone();
    let mut record: Vec<i32> = vec!();
    loop {
        if mut_ops.len() == 0 {
            break;
        }
        let handle = mut_ops.remove(0);
        if handle == "+" {
            Ops::handle_ops(Ops::PLUS, &mut record);
        }
        else if handle == "D" {
            Ops::handle_ops(Ops::DOUBLE, &mut record);
        }
        else if handle == "C" {
            Ops::handle_ops(Ops::CLEAR, &mut record);
        }
        else {
            let add_to_record: i32 = handle.parse::<i32>().unwrap();
            record.push(add_to_record);
        }
    }

    let result = record.iter().sum();
    return result;
}

enum Ops {
    INTEGER,
    PLUS,
    DOUBLE,
    CLEAR,
}

impl Ops {
    pub fn handle_ops(op: Ops, record: &mut Vec<i32>) -> Option<bool> {
         match op {
             Ops::PLUS => {
                 // add the last to elements to create a new score and push it to the record
                 let mut new_score: i32 = 0;
                 let last_element = *record.last()?;
                 let second_last_element = *record.get(record.len()-2)?;
                 new_score = last_element + second_last_element ;
                 record.push(new_score);
                 Some(true)
             }
             Ops::DOUBLE => {
                 // get last number in record, double it, push it to the record
                 let mut new_score = *record.last()? * 2;
                 record.push(new_score);
                 Some(true)
             }
             Ops::CLEAR => {
                 // remove most recent element
                 record.pop();
                 Some(true)
             }
             _ => Some(false)
         }
    }
}

#[cfg(test)]
mod tests {
    use crate::worked_problems::baseball_game::cal_points;

    #[test]
    fn baseball_works() {
        assert_eq!(30, cal_points(vec!("5","2","C","D","+")));
        assert_eq!(12, cal_points(vec!("5", "1", "+")));
        assert_eq!(24, cal_points(vec!("5", "1", "+", "D")));
        assert_eq!(12, cal_points(vec!("5", "1", "+", "D", "C")));
    }
}