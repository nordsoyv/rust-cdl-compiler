#![feature(test)]

extern crate test;
extern crate cdl_core;



#[cfg(test)]
mod tests {
    use test::{Bencher, black_box};
    use cdl_core::{select_field, select_entity, compile};

    #[bench]
    fn bench_parse(b : &mut Bencher){
        let complete_script  = .to_string();



        b.iter(|| {

            compile(complete_script.clone()).unwrap();
        });


    }

    #[bench]
    fn bench_select(b: &mut Bencher) {
        let cdl = "
   datatable kpi data1 {
      type : nps
      vpath : t1:q1
    }

    page #overview {
      widget kpi kpi1{
        type : nps
        vpath : t1:q1
        label : \"KPI\"
      }
      widget kpi kpi2{
        type : nps
        vpath : t1:q1
        label : \"KPI\"
      }

      widget account {
        type : nps
        vpath : t1:q1
        label : \"KPI\"
      }
    }
".to_string();
        let root = compile(cdl).unwrap();

        b.iter(|| {
            black_box(select_entity(&root, "page > widget[kpi]"));
            black_box(select_field(&root, "widget > .label"));
        });
    }
}
