use std::io;

#[derive(Debug)]
struct BarangTerjual {
    kode_barang: String,
    nama_barang: String,
    jumlah: u32,
}

fn tambah_barang_terjual(daftar_barang_terjual: &mut Vec<BarangTerjual>) {
    println!("Tambah Barang Terjual");

    println!("Masukkan Kode Barang:");
    let mut kode_barang = String::new();
    io::stdin()
        .read_line(&mut kode_barang)
        .expect("Gagal membaca kode barang");

    println!("Masukkan Nama Barang:");
    let mut nama_barang = String::new();
    io::stdin()
        .read_line(&mut nama_barang)
        .expect("Gagal membaca nama barang");

    let jumlah: u32 = loop {
        println!("Masukkan Jumlah Barang:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Gagal membaca jumlah barang");

        match input.trim().parse() {
            Ok(value) => break value,
            Err(_) => println!("Jumlah barang harus berupa angka. Silahkan coba lagi."),
        }
    };

    let barang_terjual_baru = BarangTerjual {
        kode_barang: kode_barang.trim().to_string(),
        nama_barang: nama_barang.trim().to_string(),
        jumlah,
    };

    daftar_barang_terjual.push(barang_terjual_baru);
    println!("Barang berhasil ditambahkan ke daftar terjual!");
}

fn tampilkan_barang_terjual(daftar_barang_terjual: &Vec<BarangTerjual>) {
    println!("Data Barang Terjual");

    for barang_terjual in daftar_barang_terjual {
        println!("{:?}", barang_terjual);
    }
}

fn main() {
    let mut daftar_barang_terjual: Vec<BarangTerjual> = Vec::new();

    loop {
        println!("Menu:");
        println!("1. Tambah Barang Terjual");
        println!("2. Tampilkan Barang Terjual");
        println!("3. Keluar");
        println!("Enter your choice:");

        let mut pilihan = String::new();
        io::stdin()
            .read_line(&mut pilihan)
            .expect("Gagal membaca pilihan");

        match pilihan.trim().parse() {
            Ok(1) => tambah_barang_terjual(&mut daftar_barang_terjual),
            Ok(2) => tampilkan_barang_terjual(&daftar_barang_terjual),
            Ok(3) => {
                println!("Keluar dari program. Selamat tinggal!");
                break;
            }
            _ => println!("Pilihan tidak valid. Silahkan coba lagi."),
        }
    }
}
