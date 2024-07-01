mod editor;
use editor::TerminalEditor;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if let Some(file_arg) = args.get(1) {
        TerminalEditor::default().run(file_arg);
    } else {
        println!("No file was given");
    }
}
