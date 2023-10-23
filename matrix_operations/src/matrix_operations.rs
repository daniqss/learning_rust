use std::vec::Vec;
use std::io;

pub struct Matrix {
    rows: usize,
    columns: usize,
    data: Vec<usize>,
}

impl Matrix {
    pub fn new() -> Matrix {
        
        let (rows, columns) = Self::ask_size();
        Matrix {
            rows: rows,
            columns: columns,
            data: vec![0; (rows * columns) as usize],
        }
    }


    fn ask_size () -> (usize, usize) {
        let mut rows_input = String::new();
        let mut columns_input = String::new();

        println!("Rows: ");
        io::stdin().read_line(&mut rows_input).expect("Error al leer las filas");
        let rows: usize = rows_input.trim().parse().expect("Error al analizar las filas");
    
        println!("Columns: ");
        io::stdin().read_line(&mut columns_input).expect("Error al leer las columnas");
        let columns: usize = columns_input.trim().parse().expect("Error al analizar las columnas");
    

        (rows, columns)
    }

    pub fn print(&self) {
        for i in 0..self.rows {
            for j in 0..self.columns {
                print!("{} ", self.data[(i * self.columns + j) as usize]);
            }
            println!();
        }
    }

    pub fn to_array2d (&self) -> [[i32; Self.rows as usize]; Self.columns as usize] {
        let mut array2d: [[i32; self.rows]; self.columns];

        for i in 0..self.rows {
            for j in 0..self.columns {
                let index = (i * self.columns + j)
                array2d[i][j] = self.data[index];
            }
        }

        array2d
    }
}
