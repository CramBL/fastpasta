pub struct Stats {
    pub rdhs_seen: u64,
    pub rdhs_filtered: u64,
    pub payload_size: u64,
    pub links_observed: Vec<u8>,
    pub processing_time: std::time::Instant,
}
impl Stats {
    pub fn new() -> Self {
        Stats {
            rdhs_seen: 0,
            rdhs_filtered: 0,
            payload_size: 0,
            links_observed: vec![],
            processing_time: std::time::Instant::now(),
        }
    }
    pub fn print(&self) {
        println!("Total RDHs: {}", self.rdhs_seen);
        println!("Total RDHs filtered: {}", self.rdhs_filtered);
        println!("Total payload size: {}", self.payload_size);
        println!("Links observed: {:?}", self.links_observed);
        println!("Processing time: {:?}", self.processing_time.elapsed());
    }
    pub fn print_time(&self) {
        println!("Processing time: {:?}", self.processing_time.elapsed());
    }
}