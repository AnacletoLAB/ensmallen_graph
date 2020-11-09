
use std::fs;
use std::fs::File;
use graph_harness::*;
use arbitrary_rust::Arbitrary;
use std::io::prelude::*;

const FROM_CSV_FOLDER: &str = "../corpus/from_csv";
const FROM_VEC_FOLDER: &str = "../corpus/from_vec";

macro_rules! serialize {
    ($filename:expr, csv, $obj:expr) => {{
        let path = format!("{}/{}", FROM_CSV_FOLDER, $filename);
        let mut buffer = std::fs::File::create(&path).unwrap();
        buffer.write_all(&$obj.to_bytes()).unwrap();

        let mut f = File::open(&path).expect(&format!("File found - {}", &path));
        let metadata = fs::metadata(&path).expect("unable to read metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        f.read(&mut buffer).expect("buffer overflow");

        assert_eq!($obj, FromCsvHarnessParams::from_bytes(buffer), "{} is not reproducible!!!!", path);

    }};
    ($filename:expr, vec, $obj:expr) => {{
        let path = format!("{}/{}", FROM_VEC_FOLDER, $filename);
        let mut buffer = std::fs::File::create(&path).unwrap();
        buffer.write_all(&$obj.to_bytes()).unwrap();

        let mut f = File::open(&path).expect(&format!("File found - {}", &path));
        let metadata = fs::metadata(&path).expect("unable to read metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        f.read(&mut buffer).expect("buffer overflow");

        assert_eq!($obj, FromVecHarnessParams::from_bytes(buffer), "{} is not reproducible!!!!", path);
    }};
}

fn gen_cases(name: &str, mut data: FromCsvHarnessParams){
    serialize!(format!("{}_full.corpus", name), csv, data);
    if let Some(nodes_reader) = &mut data.nodes_reader {
        if nodes_reader.node_types_column.is_some() || nodes_reader.node_types_column_number.is_some(){
            nodes_reader.node_types_column_number = None;
            nodes_reader.node_types_column = None;
        }
        serialize!(format!("{}_no_node_types.corpus", name), csv, data);
        data.nodes_reader = None;
        serialize!(format!("{}_no_nodes.corpus", name), csv, data);
    }

    if data.edge_reader.weights_column.is_some() || data.edge_reader.weights_column_number.is_some() {
        data.edge_reader.weights_column_number = None;
        data.edge_reader.weights_column = None;
        serialize!(format!("{}_no_weights.corpus", name), csv, data);
    }
    
    if data.edge_reader.edge_types_column.is_some() || data.edge_reader.edge_types_column_number.is_some() {
        data.edge_reader.edge_types_column_number = None;
        data.edge_reader.edge_types_column = None;
        serialize!(format!("{}_no_edge_types.corpus", name), csv, data);
    }
}

fn main() {
    let ppi_nodes =    fs::read_to_string("../../graph/tests/data/ppi/nodes.tsv").unwrap();
    let ppi_edges =    fs::read_to_string("../../graph/tests/data/ppi/edges.tsv").unwrap();
    let macaco_edges = fs::read_to_string("../../graph/tests/data/macaque.tsv").unwrap();

    let regressions = (1..=18).map(|i| {
        let path = format!("../../graph/tests/data/regression/{}.tsv", i);
        fs::read_to_string(path).unwrap()
    }).collect::<Vec<String>>();

    gen_cases("ppi", FromCsvHarnessParams{
        directed: false,
        nodes_reader: Some(
            NodeFileReaderParams{
                file: ppi_nodes.clone(),
                reader: CSVFileReaderParams{
                    verbose: Some(false),
                    separator: Some("\t".to_string()),
                    header: Some(true),
                    rows_to_skip: Some(0),
                    ignore_duplicates: Some(true)
                },
                default_node_type: Some("default".to_string()),
                nodes_column_number: Some(0),
                nodes_column: Some("id".to_string()),
                node_types_column: Some("category".to_string()),
                node_types_column_number: Some(1)
            }
        ),
        edge_reader: EdgeFileReaderParams{
            file: ppi_edges.clone(),
            reader: CSVFileReaderParams{
                verbose: Some(false),
                separator: Some("\t".to_string()),
                header: Some(true),
                rows_to_skip: Some(0),
                ignore_duplicates: Some(true)
            },
            sources_column_number: Some(0),
            sources_column: Some("subject".to_string()),
            destinations_column_number: Some(1),
            destinations_column: Some("object".to_string()),
            edge_types_column_number: Some(2),
            edge_types_column: Some("edge_label".to_string()),
            weights_column_number: Some(3),
            weights_column: Some("weight".to_string()),
            default_weight: Some(5.0),
            default_edge_type: Some("Kebab".to_string()),
            skip_self_loops: Some(false),
        }
    });

    gen_cases("macaque", FromCsvHarnessParams{
        directed: false,
        nodes_reader: None,
        edge_reader: EdgeFileReaderParams{
            file: macaco_edges.clone(),
            reader: CSVFileReaderParams{
                verbose: Some(false),
                separator: Some("\t".to_string()),
                header: Some(false),
                rows_to_skip: Some(0),
                ignore_duplicates: Some(true)
            },
            sources_column_number: Some(0),
            sources_column: None,
            destinations_column_number: Some(1),
            destinations_column: None,
            edge_types_column_number: None,
            edge_types_column: None,
            weights_column_number: None,
            weights_column: None,
            default_weight: Some(5.0),
            default_edge_type: None,
            skip_self_loops: Some(false),
        }
    });

    gen_cases("1", FromCsvHarnessParams{
        directed: false,
        nodes_reader: None,
        edge_reader: EdgeFileReaderParams{
            file: regressions[0].clone(),
            reader: CSVFileReaderParams{
                verbose: Some(false),
                separator: Some("\t".to_string()),
                header: Some(false),
                rows_to_skip: Some(0),
                ignore_duplicates: Some(true)
            },
            sources_column_number: Some(0),
            sources_column: None,
            destinations_column_number: Some(1),
            destinations_column: None,
            edge_types_column_number: None,
            edge_types_column: None,
            weights_column_number: None,
            weights_column: None,
            default_weight: None,
            default_edge_type: None,
            skip_self_loops: Some(false),
        }
    });

    gen_cases("2", FromCsvHarnessParams{
        directed: false,
        nodes_reader: None,
        edge_reader: EdgeFileReaderParams{
            file: regressions[1].clone(),
            reader: CSVFileReaderParams{
                verbose: Some(false),
                separator: Some("\t".to_string()),
                header: Some(false),
                rows_to_skip: Some(0),
                ignore_duplicates: Some(true)
            },
            sources_column_number: Some(0),
            sources_column: None,
            destinations_column_number: Some(1),
            destinations_column: None,
            edge_types_column_number: None,
            edge_types_column: None,
            weights_column_number: None,
            weights_column: None,
            default_weight: None,
            default_edge_type: None,
            skip_self_loops: Some(false),
        }
    });

    gen_cases("3", FromCsvHarnessParams{
        directed: false,
        nodes_reader: None,
        edge_reader: EdgeFileReaderParams{
            file: regressions[2].clone(),
            reader: CSVFileReaderParams{
                verbose: Some(false),
                separator: Some("\t".to_string()),
                header: Some(false),
                rows_to_skip: Some(0),
                ignore_duplicates: Some(true)
            },
            sources_column_number: Some(0),
            sources_column: None,
            destinations_column_number: Some(1),
            destinations_column: None,
            edge_types_column_number: Some(3),
            edge_types_column: None,
            weights_column_number: None,
            weights_column: None,
            default_weight: None,
            default_edge_type: None,
            skip_self_loops: Some(false),
        }
    });

    gen_cases("4", FromCsvHarnessParams{
        directed: false,
        nodes_reader: None,
        edge_reader: EdgeFileReaderParams{
            file: regressions[3].clone(),
            reader: CSVFileReaderParams{
                verbose: Some(false),
                separator: Some("\t".to_string()),
                header: Some(false),
                rows_to_skip: Some(0),
                ignore_duplicates: Some(true)
            },
            sources_column_number: Some(0),
            sources_column: None,
            destinations_column_number: Some(1),
            destinations_column: None,
            edge_types_column_number: None,
            edge_types_column: None,
            weights_column_number: None,
            weights_column: None,
            default_weight: None,
            default_edge_type: None,
            skip_self_loops: Some(false),
        }
    });

    gen_cases("5", FromCsvHarnessParams{
        directed: false,
        nodes_reader: None,
        edge_reader: EdgeFileReaderParams{
            file: regressions[4].clone(),
            reader: CSVFileReaderParams{
                verbose: Some(false),
                separator: Some(",".to_string()),
                header: Some(false),
                rows_to_skip: Some(0),
                ignore_duplicates: Some(true)
            },
            sources_column_number: Some(0),
            sources_column: None,
            destinations_column_number: Some(1),
            destinations_column: None,
            edge_types_column_number: None,
            edge_types_column: None,
            weights_column_number: None,
            weights_column: None,
            default_weight: None,
            default_edge_type: None,
            skip_self_loops: Some(false),
        }
    });

    gen_cases("6", FromCsvHarnessParams{
        directed: false,
        nodes_reader: None,
        edge_reader: EdgeFileReaderParams{
            file: regressions[5].clone(),
            reader: CSVFileReaderParams{
                verbose: Some(false),
                separator: Some(",".to_string()),
                header: Some(false),
                rows_to_skip: Some(0),
                ignore_duplicates: Some(true)
            },
            sources_column_number: Some(0),
            sources_column: None,
            destinations_column_number: Some(1),
            destinations_column: None,
            edge_types_column_number: None,
            edge_types_column: None,
            weights_column_number: None,
            weights_column: None,
            default_weight: None,
            default_edge_type: None,
            skip_self_loops: Some(false),
        }
    });

    gen_cases("7", FromCsvHarnessParams{
        directed: false,
        nodes_reader: None,
        edge_reader: EdgeFileReaderParams{
            file: regressions[6].clone(),
            reader: CSVFileReaderParams{
                verbose: Some(false),
                separator: Some(",".to_string()),
                header: Some(false),
                rows_to_skip: Some(0),
                ignore_duplicates: Some(true)
            },
            sources_column_number: Some(0),
            sources_column: None,
            destinations_column_number: Some(1),
            destinations_column: None,
            edge_types_column_number: None,
            edge_types_column: None,
            weights_column_number: Some(2),
            weights_column: None,
            default_weight: None,
            default_edge_type: None,
            skip_self_loops: Some(false),
        }
    });

    gen_cases("8", FromCsvHarnessParams{
        directed: false,
        nodes_reader: None,
        edge_reader: EdgeFileReaderParams{
            file: regressions[7].clone(),
            reader: CSVFileReaderParams{
                verbose: Some(false),
                separator: Some(",".to_string()),
                header: Some(false),
                rows_to_skip: Some(0),
                ignore_duplicates: Some(true)
            },
            sources_column_number: Some(0),
            sources_column: None,
            destinations_column_number: Some(1),
            destinations_column: None,
            edge_types_column_number: Some(2),
            edge_types_column: None,
            weights_column_number: Some(3),
            weights_column: None,
            default_weight: None,
            default_edge_type: None,
            skip_self_loops: Some(false),
        }
    });

    gen_cases("9", FromCsvHarnessParams{
        directed: false,
        nodes_reader: None,
        edge_reader: EdgeFileReaderParams{
            file: regressions[8].clone(),
            reader: CSVFileReaderParams{
                verbose: Some(false),
                separator: Some(",".to_string()),
                header: Some(false),
                rows_to_skip: Some(0),
                ignore_duplicates: Some(true)
            },
            sources_column_number: Some(0),
            sources_column: None,
            destinations_column_number: Some(1),
            destinations_column: None,
            edge_types_column_number: None,
            edge_types_column: None,
            weights_column_number: None,
            weights_column: None,
            default_weight: None,
            default_edge_type: None,
            skip_self_loops: Some(false),
        }
    });

    gen_cases("10", FromCsvHarnessParams{
        directed: false,
        nodes_reader: None,
        edge_reader: EdgeFileReaderParams{
            file: regressions[9].clone(),
            reader: CSVFileReaderParams{
                verbose: Some(false),
                separator: Some(" ".to_string()),
                header: Some(false),
                rows_to_skip: Some(0),
                ignore_duplicates: Some(true)
            },
            sources_column_number: Some(0),
            sources_column: None,
            destinations_column_number: Some(1),
            destinations_column: None,
            edge_types_column_number: None,
            edge_types_column: None,
            weights_column_number: Some(2),
            weights_column: None,
            default_weight: None,
            default_edge_type: None,
            skip_self_loops: Some(false),
        }
    });

    gen_cases("11", FromCsvHarnessParams{
        directed: false,
        nodes_reader: None,
        edge_reader: EdgeFileReaderParams{
            file: regressions[10].clone(),
            reader: CSVFileReaderParams{
                verbose: Some(false),
                separator: Some(",".to_string()),
                header: Some(false),
                rows_to_skip: Some(0),
                ignore_duplicates: Some(true)
            },
            sources_column_number: Some(0),
            sources_column: None,
            destinations_column_number: Some(1),
            destinations_column: None,
            edge_types_column_number: None,
            edge_types_column: None,
            weights_column_number: None,
            weights_column: None,
            default_weight: None,
            default_edge_type: None,
            skip_self_loops: Some(false),
        }
    });

    gen_cases("12", FromCsvHarnessParams{
        directed: false,
        nodes_reader: None,
        edge_reader: EdgeFileReaderParams{
            file: regressions[11].clone(),
            reader: CSVFileReaderParams{
                verbose: Some(false),
                separator: Some(",".to_string()),
                header: Some(false),
                rows_to_skip: Some(0),
                ignore_duplicates: Some(true)
            },
            sources_column_number: Some(0),
            sources_column: None,
            destinations_column_number: Some(1),
            destinations_column: None,
            edge_types_column_number: Some(2),
            edge_types_column: None,
            weights_column_number: None,
            weights_column: None,
            default_weight: None,
            default_edge_type: None,
            skip_self_loops: Some(false),
        }
    });

    gen_cases("13", FromCsvHarnessParams{
        directed: false,
        nodes_reader: None,
        edge_reader: EdgeFileReaderParams{
            file: regressions[12].clone(),
            reader: CSVFileReaderParams{
                verbose: Some(false),
                separator: Some(",".to_string()),
                header: Some(false),
                rows_to_skip: Some(0),
                ignore_duplicates: Some(true)
            },
            sources_column_number: Some(0),
            sources_column: None,
            destinations_column_number: Some(1),
            destinations_column: None,
            edge_types_column_number: None,
            edge_types_column: None,
            weights_column_number: Some(2),
            weights_column: None,
            default_weight: None,
            default_edge_type: None,
            skip_self_loops: Some(false),
        }
    });

    gen_cases("14", FromCsvHarnessParams{
        directed: false,
        nodes_reader: None,
        edge_reader: EdgeFileReaderParams{
            file: regressions[13].clone(),
            reader: CSVFileReaderParams{
                verbose: Some(false),
                separator: Some(",".to_string()),
                header: Some(false),
                rows_to_skip: Some(0),
                ignore_duplicates: Some(true)
            },
            sources_column_number: Some(0),
            sources_column: None,
            destinations_column_number: Some(1),
            destinations_column: None,
            edge_types_column_number: Some(2),
            edge_types_column: None,
            weights_column_number: None,
            weights_column: None,
            default_weight: None,
            default_edge_type: None,
            skip_self_loops: Some(false),
        }
    });

    gen_cases("15", FromCsvHarnessParams{
        directed: false,
        nodes_reader: None,
        edge_reader: EdgeFileReaderParams{
            file: regressions[14].clone(),
            reader: CSVFileReaderParams{
                verbose: Some(false),
                separator: Some(",".to_string()),
                header: Some(false),
                rows_to_skip: Some(0),
                ignore_duplicates: Some(true)
            },
            sources_column_number: Some(0),
            sources_column: None,
            destinations_column_number: Some(1),
            destinations_column: None,
            edge_types_column_number: Some(2),
            edge_types_column: None,
            weights_column_number: None,
            weights_column: None,
            default_weight: None,
            default_edge_type: None,
            skip_self_loops: Some(false),
        }
    });

    gen_cases("16", FromCsvHarnessParams{
        directed: false,
        nodes_reader: None,
        edge_reader: EdgeFileReaderParams{
            file: regressions[15].clone(),
            reader: CSVFileReaderParams{
                verbose: Some(false),
                separator: Some(",".to_string()),
                header: Some(false),
                rows_to_skip: Some(0),
                ignore_duplicates: Some(true)
            },
            sources_column_number: Some(0),
            sources_column: None,
            destinations_column_number: Some(1),
            destinations_column: None,
            edge_types_column_number: Some(2),
            edge_types_column: None,
            weights_column_number: Some(3),
            weights_column: None,
            default_weight: None,
            default_edge_type: None,
            skip_self_loops: Some(false),
        }
    });

    gen_cases("17", FromCsvHarnessParams{
        directed: false,
        nodes_reader: None,
        edge_reader: EdgeFileReaderParams{
            file: regressions[16].clone(),
            reader: CSVFileReaderParams{
                verbose: Some(false),
                separator: Some(",".to_string()),
                header: Some(false),
                rows_to_skip: Some(0),
                ignore_duplicates: Some(true)
            },
            sources_column_number: Some(0),
            sources_column: None,
            destinations_column_number: Some(1),
            destinations_column: None,
            edge_types_column_number: Some(2),
            edge_types_column: None,
            weights_column_number: Some(3),
            weights_column: None,
            default_weight: None,
            default_edge_type: None,
            skip_self_loops: Some(false),
        }
    });

    gen_cases("18", FromCsvHarnessParams{
        directed: false,
        nodes_reader: None,
        edge_reader: EdgeFileReaderParams{
            file: regressions[17].clone(),
            reader: CSVFileReaderParams{
                verbose: Some(false),
                separator: Some(",".to_string()),
                header: Some(false),
                rows_to_skip: Some(0),
                ignore_duplicates: Some(true)
            },
            sources_column_number: Some(0),
            sources_column: None,
            destinations_column_number: Some(1),
            destinations_column: None,
            edge_types_column_number: Some(2),
            edge_types_column: None,
            weights_column_number: Some(3),
            weights_column: None,
            default_weight: None,
            default_edge_type: None,
            skip_self_loops: Some(false),
        }
    });



    

    
    
}