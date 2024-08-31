use std::sync::atomic::{AtomicU32, Ordering};

pub trait Printer {
    fn print(&self, msg: String);
}

#[derive(Debug, Default)]
struct PlainPrinter;

impl Printer for PlainPrinter {
    fn print(&self, msg: String) {
        println!("Plain: {msg}");
    }
}

#[derive(Debug)]
struct PrintCounter<P> {
    count: AtomicU32,
    printer: P,
}

impl<P> PrintCounter<P>
where
    P: Printer,
{
    pub fn new(printer: P) -> Self {
        Self {
            count: AtomicU32::default(),
            printer,
        }
    }
}

impl<P> Printer for PrintCounter<P>
where
    P: Printer,
{
    fn print(&self, msg: String) {
        self.count.fetch_add(1, Ordering::SeqCst);
        print!("{}. ", self.count.load(Ordering::SeqCst));
        self.printer.print(msg);
    }
}

#[derive(Debug)]
struct DecoratedPrinter<P> {
    printer: P,
}

impl<P> DecoratedPrinter<P>
where
    P: Printer,
{
    pub fn new(printer: P) -> Self {
        Self { printer }
    }
}

impl<P> Printer for DecoratedPrinter<P>
where
    P: Printer,
{
    fn print(&self, msg: String) {
        println!("=================================");
        self.printer.print(msg);
        println!("=================================");
    }
}

fn print_message<P>(printer: &P, msg: String)
where
    P: Printer,
{
    printer.print(msg);
}

fn main() {
    let messenger = DecoratedPrinter::new(PrintCounter::new(PlainPrinter));
    print_message(&messenger, "hello".to_string());
    print_message(&messenger, "goodbye".to_string());
}
