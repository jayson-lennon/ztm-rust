use std::sync::atomic::{AtomicU32, Ordering};

pub trait Printer {
    fn print(&self, msg: String);
}

fn print_message<P>(printer: &P, msg: String)
where
    P: Printer,
{
    printer.print(msg);
}

struct PlainPrinter;

impl Printer for PlainPrinter {
    fn print(&self, msg: String) {
        println!("Plain: {msg}");
    }
}

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
        self.count.fetch_add(1, Ordering::Relaxed);
        print!("{}. ", self.count.load(Ordering::Relaxed));
        self.printer.print(msg)
    }
}

struct SurroundingPrinter<P> {
    printer: P,
}

impl<P> SurroundingPrinter<P>
where
    P: Printer,
{
    pub fn new(printer: P) -> Self {
        Self { printer }
    }
}

impl<P> Printer for SurroundingPrinter<P>
where
    P: Printer,
{
    fn print(&self, msg: String) {
        println!("==============================");
        self.printer.print(msg);
        println!("==============================");
    }
}

fn main() {
    let messenger = SurroundingPrinter::new(PrintCounter::new(PlainPrinter));
    print_message(&messenger, "hello".to_string());
    print_message(&messenger, "goodbye".to_string());
}
