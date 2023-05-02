use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;

struct Atom {
    atomname: String,
    x: f64,
    y: f64,
    z: f64,
}

struct Residue {
    chainid: String,
    resnum: i32,
    resname: String,
    atoms: Vec<Atom>,
}

fn calculate_contacts(pdb: Vec<String>, cutoff: f64) {
    let mut residues = HashMap::<(String, i32), Residue>::new();

    for line in pdb {
        // If the line is a record of an atom
        if line.starts_with("ATOM") {
            // Ignore Hydrogen atoms
            let element: String = line[76..78].to_string().trim().to_string();
            if element == "H" {
                continue;
            }

            // Get the atom's coordinates
            let x: f64 = line[30..38].trim().parse::<f64>().unwrap();
            let y: f64 = line[38..46].trim().parse::<f64>().unwrap();
            let z: f64 = line[46..54].trim().parse::<f64>().unwrap();

            // Get the chainID
            let chainid: String = line[21..22].to_string().trim().to_string();

            // Get the resnum
            let resnum: i32 = line[22..26].trim().parse::<i32>().unwrap();

            // Get the resname
            let resname: String = line[17..20].to_string().trim().to_string();

            // Get the atomname
            let atomname: String = line[12..16].to_string().trim().to_string();

            //
            let res = residues
                .entry((chainid.clone(), resnum.clone()))
                .or_insert(Residue {
                    chainid,
                    resnum,
                    resname,
                    atoms: vec![],
                });
            res.atoms.push(Atom { atomname, x, y, z });
        }
    }

    for res_a in residues.values() {
        for res_b in residues.values() {
            // Only calculate for residues in different chains
            if res_a.chainid == res_b.chainid {
                continue;
            }

            // Calculate the euclidean distance between the atoms
            for atom_a in res_a.atoms.iter() {
                for atom_b in res_b.atoms.iter() {
                    let dist: f64 = ((atom_a.x - atom_b.x).powi(2)
                        + (atom_a.y - atom_b.y).powi(2)
                        + (atom_a.z - atom_b.z).powi(2))
                    .sqrt();
                    if dist <= cutoff {
                        println!(
                            "{} {} {} {} {} {} {} {} {:.3}",
                            res_a.resname,
                            atom_a.atomname,
                            res_a.chainid,
                            res_a.resnum,
                            res_b.resname,
                            atom_b.atomname,
                            res_b.chainid,
                            res_b.resnum,
                            dist
                        );
                    }
                }
            }
        }
    }
}

fn load_file(pdbf: &String) -> Vec<String> {
    let file = File::open(pdbf).expect("Cannot open file");

    let mut atom_lines = Vec::<String>::new();
    for line in BufReader::new(file).lines() {
        if let Ok(line) = line {
            atom_lines.push(line);
        };
    }
    atom_lines
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("usage: <exec> <input.pdb> <cutoff>");
        exit(1)
    }

    let input_arg = &args[1];
    let cutoff_arg = &args[2];

    let cutoff = cutoff_arg
        .parse()
        .expect("Could not parse cutoff, make sure its a number.");

    let pdb_vec = load_file(&input_arg);
    calculate_contacts(pdb_vec, cutoff);
}
