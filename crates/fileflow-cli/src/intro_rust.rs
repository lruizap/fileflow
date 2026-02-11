pub fn intro_rust() {
    println!("==============================");
    println!(" 🦀 INTRODUCCIÓN BÁSICA A RUST");
    println!("==============================\n");

    variables();
    strings();
    arrays_y_vectores();
    structs();
    enums_y_match();
    option_y_result();

    println!("\n✔ Listo. Ya conoces lo esencial para empezar FileFlow.");
}

/* ---------------- VARIABLES ---------------- */

fn variables() {
    println!("--- Variables ---");

    let mut contador = 0;
    contador += 1;

    println!("contador = {}", contador);
}

/* ---------------- STRINGS ---------------- */

fn strings() {
    println!("\n--- Strings ---");

    let mut nombre = String::from("FileFlow");
    nombre.push_str(" App");

    println!("{}", nombre);
}

/* ---------------- ARRAYS Y VECTORES ---------------- */

fn arrays_y_vectores() {
    println!("\n--- Array fijo ---");

    let nums = [1, 2, 3];

    for n in nums {
        println!("num: {}", n);
    }

    println!("\n--- Vector dinámico ---");

    let mut archivos = vec!["a.txt", "b.txt"];
    archivos.push("c.txt");

    for (i, f) in archivos.iter().enumerate() {
        println!("{} -> {}", i, f);
    }
}

/* ---------------- STRUCT ---------------- */

fn structs() {
    println!("\n--- Struct ---");

    struct Job {
        id: u32,
        name: String,
    }

    let job = Job {
        id: 1,
        name: "backup".into(),
    };

    println!("Job {} - {}", job.id, job.name);
}

/* ---------------- ENUM + MATCH ---------------- */

fn enums_y_match() {
    println!("\n--- Enum + match ---");

    enum Status {
        Pending,
        Running,
        Done,
    }

    let estados = [Status::Pending, Status::Running, Status::Done];

    for estado in estados {
        match estado {
            Status::Pending => println!("Pendiente"),
            Status::Running => println!("Ejecutando"),
            Status::Done => println!("Finalizado"),
        }
    }
}

/* ---------------- OPTION + RESULT ---------------- */

fn option_y_result() {
    println!("\n--- Option ---");

    let valor: Option<i32> = Some(5);

    match valor {
        Some(v) => println!("valor = {}", v),
        None => println!("sin valor"),
    }

    println!("\n--- Result ---");

    fn dividir(a: i32, b: i32) -> Result<i32, &'static str> {
        if b == 0 {
            Err("división por cero")
        } else {
            Ok(a / b)
        }
    }

    match dividir(10, 2) {
        Ok(v) => println!("resultado = {}", v),
        Err(e) => println!("error: {}", e),
    }
}
