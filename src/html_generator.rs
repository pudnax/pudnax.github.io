use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use structopt::StructOpt;

fn main() {
    let args = Cli::from_args();

    println!("input arguments:\n\tpattern->{}", args.pattern);

    for string in args.pattern.split(',') {
        if string.contains("..") {
            let range: Vec<_> = string
                .split("..")
                .map(|x| x.parse::<i64>().expect("can't convert"))
                .collect();
            for i in range[0]..range[range.len() - 1] {
                create_html(i);
            }
        } else {
            create_html(string.parse::<i64>().expect("can't convert"));
        }
    }
}

fn create_html(num: i64) {
    let text = format!(
        r#"
{or}
            <h2 id="draggable"> Num: {:03} </h2>
            <div id="obj" draggable="true" class="btn-group">
                <input type="button" onclick="location.href='./{:03}.html';" value="Prev" />
                <input type="button" onclick="location.href='./{:03}.html';" value="Next" />
            </div>
{end}
"#,
        num,
        num - 1,
        num + 1,
        or = ORIGIN,
        end = END
    );
    let name = format!("html/{:03}.html", &num);
    let path = Path::new(&name);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    match file.write_all(text.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}

#[derive(StructOpt)]
struct Cli {
    pattern: String,
}

static ORIGIN: &str = r#"
<!DOCTYPE html>
<html style="background-color:black;color:white">

<head>
    <meta charset="utf-8" />
    <meta http-equiv="X-UA-Compatible" content="IE=edge" />
    <meta content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=1" name="viewport" />
    <style>
        #draggable {
            margin-top: 5px;
            margin-bottom: 5px;
            margin-right: 25px;
            padding: 0.0em;
        }
        
        #obj {
            margin-top: 5px;
            margin-bottom: 5px;
            margin-right: 25px;
            padding: 0.0em;
        }
    </style>
    <script src="https://code.jquery.com/jquery-1.12.4.js"></script>
    <script src="https://code.jquery.com/ui/1.12.1/jquery-ui.js"></script>
    #<script src="../libs/support.js"></script>

</head>
<body>
    <script src="my_lyon.js"></script>
    <div id="obj" draggable="true" style='float: right'>
        <!-- <button onclick="hide()">Hide</button>
        <button id="obj" onclick="hide_all()">Hide all</button> -->
        <div id="draggable">"#;

static END: &str = r#"
            <div draggable="true" id="obj">
                <div id="obj" style="display: block;">
                    <b>Press H to hide</b>
                </div>
                <div style="display: block;">
                    <b>Description</b>
                </div>
            </div>
        </div>
    </div>
</body>

</html>"#;
