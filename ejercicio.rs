use std::fs::File;
use std::io::{self, BufRead};
use std::io::Read;
use std::path::Path;
use std::fs::OpenOptions;
use std::f32;
use std::io::Write;

fn leer_archivo(mut f: &File) {
    let mut texto = String::new();
    f.read_to_string(&mut texto).unwrap();
    println!("{}", &texto);
}

fn crear_archivo(p: &Path) {
    let _archivo = File::create(p).expect("El archivo no pudo crearse");
    println!("El archivo fue creado");
}


fn leer_lineas<P>(nombre_archivo: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let archivo = File::open(nombre_archivo)?;
    Ok(io::BufReader::new(archivo).lines())
}

fn editar_archivo(p: &Path) -> File{
    let mut vinculo = OpenOptions::new();
    let vinculo = vinculo.append(true);
    let archivo = match vinculo.open(p){
        Err(_why) => panic!("No se puede abrir el archivo"),
        Ok(archivo) => archivo,
    };

    return archivo
}

fn open_file(p: &Path) {
    if Path::new(p).exists(){
        let _archivo = match File::open(&p){
            Err(_why) => panic!("El archivo no se puede abrir..."),
            Ok(archivo) => archivo,
        };
    } else {
        crear_archivo(p);
        panic!("reinicie, porfavor")
    }
}

fn main() {
    let path = Path::new("reporte.txt");
    open_file(path);
    let mut archivo = editar_archivo(path);

    if let Ok(lines) = leer_lineas("./notas.txt") {
        let mut nombres:u32 = 0;
        for line in lines {
            if let Ok(ip) = line {
                let split = ip.split(":");
                let mut suma:f32 = 0.0;
                let mut contador:u32 = 0;

                for s in split {
                    if contador == 0 && nombres == 0 {
                        println!("{}:", s);
                        archivo.write_all(b"Pepito ");
                    } else if contador == 0 && nombres == 1 {
                        println!("{}:", s);
                        archivo.write_all(b"Juanito ");
                    } else if contador == 0 && nombres == 2 {
                        println!("{}:", s);
                        archivo.write_all(b"Maria ");
                    } else if  contador == 0 && nombres == 3 {
                        println!("{}:", s);
                        archivo.write_all(b"Joselito ");
                    }else {
                        let num: f32 = match s.parse::<f32>() {
                            Err(_) => panic!("No es un numero compatible"),
                            Ok(s) => s
                        };

                        suma += num;
                    }

                    let promedio:f32 = suma/6.0;

                    if promedio < 4.0 && contador == 6{
                        println!("Reprobo, con un promedio {}", promedio);
                        archivo.write_all(b"reprobo\n");
                    } else if promedio >= 4.0 && contador == 6 {
                        println!("Aprobo, con un promedio {}", promedio);
                        archivo.write_all(b"aprobo\n");
                    }

                    contador += 1;
                }
            }
            println!("");
            nombres += 1;
        }
    }
    
    archivo.write_all(b"\n");
    println!("");
    println!("");
    println!("");
    leer_archivo(&archivo)
}
