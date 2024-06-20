mod ownership;
mod references;
mod slicing;

fn main() {
    // running the ownership examples
    ownership::ownership_examples();

    // running the references examples
    references::references_examples();

    // running the slicing examples
    slicing::slicing_examples()
}
