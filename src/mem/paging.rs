use core::marker::PhantomData;

#[repr(C)]
pub struct Table {
    pub entries: [TableEntry; 512]
}


#[repr(C)]
pub struct TableEntry {

}

pub fn id_map(start: usize, end: usize) {
}
