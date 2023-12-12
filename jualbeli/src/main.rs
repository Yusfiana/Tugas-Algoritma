use std::io;
use cli_tables::Table;
#[derive(Debug)]
struct BarangTerjual {
    kode_barang: String,
    nama_barang: String,
    jumlah: u32,
}

fn tambah_barang_terjual(daftar_barang_terjual: &mut Vec<BarangTerjual>) {
    println!("Tambah Barang Terjual");

    let kode_barang: String = get_user_input("Masukkan Kode Barang");
    let nama_barang: String = get_user_input("Masukkan Nama Barang");

    let jumlah: u32 = loop {
        let input: String = get_user_input("Masukkan Jumlah Barang");
        match input.trim().parse::<u32>() {
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




fn tampilkan_barang_terjual(daftar_barang_terjual: &[BarangTerjual]) {
    println!("Data Barang Terjual");
    let mut table = Table::new();
    for (index, barang_terjual) in daftar_barang_terjual.iter().enumerate()
     {
        println!("{}. {:?}", index + 1, barang_terjual);
    }
    let values = vec![
            vec!["No", "Kode Barang", "Nama Barang", "Jumlah"],
            vec!["1", "123", "sabun", "20"]
            
            

];
    table.push_rows(&values);
    
      println!("{}", table.to_string());

}






fn edit_barang_terjual(daftar_barang_terjual: &mut Vec<BarangTerjual>) {
    println!("Edit Barang Terjual");

    if daftar_barang_terjual.is_empty() {
        println!("Daftar barang terjual kosong. Tidak ada yang dapat diedit.");
        return;
    }

    tampilkan_barang_terjual(daftar_barang_terjual);

    let index: usize = loop {
        let input: String = get_user_input("Masukkan nomor barang yang ingin diedit (0 untuk batal)");
        match input.trim().parse::<usize>() {
            Ok(value) if value == 0 => return,
            Ok(value) if value <= daftar_barang_terjual.len() => break value - 1,
            _ => println!("Nomor barang tidak valid. Silahkan coba lagi."),
        }
    };

    let barang_terjual = &mut daftar_barang_terjual[index];

    println!("Masukkan informasi baru:");

    barang_terjual.kode_barang = get_user_input("Masukkan Kode Barang").trim().to_string();
    barang_terjual.nama_barang = get_user_input("Masukkan Nama Barang").trim().to_string();

    let jumlah: u32 = loop {
        let input: String = get_user_input("Masukkan Jumlah Barang");
        match input.trim().parse::<u32>() {
            Ok(value) => break value,
            Err(_) => println!("Jumlah barang harus berupa angka. Silahkan coba lagi."),
        }
    };

    barang_terjual.jumlah = jumlah;

    println!("Barang berhasil diubah!");
}

fn hapus_barang_terjual(daftar_barang_terjual: &mut Vec<BarangTerjual>) {
    println!("Hapus Barang Terjual");

    if daftar_barang_terjual.is_empty() {
        println!("Daftar barang terjual kosong. Tidak ada yang dapat dihapus.");
        return;
    }

    tampilkan_barang_terjual(daftar_barang_terjual);

    let index: usize = loop {
        let input: String = get_user_input("Masukkan nomor barang yang ingin dihapus (0 untuk batal)");
        match input.trim().parse::<usize>() {
            Ok(value) if value == 0 => return,
            Ok(value) if value <= daftar_barang_terjual.len() => break value - 1,
            _ => println!("Nomor barang tidak valid. Silahkan coba lagi."),
        }
    };

    daftar_barang_terjual.remove(index);

    println!("Barang berhasil dihapus!");
}

fn get_user_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Gagal membaca input");
    input.trim().to_string()
}

fn main() {
    let mut daftar_barang_terjual: Vec<BarangTerjual> = Vec::new();

    loop {
        println!("Menu:");
        println!("1. Tambah Barang Terjual");
        println!("2. Tampilkan Barang Terjual");
        println!("3. Edit Barang Terjual");
        println!("4. Hapus Barang Terjual");
        println!("5. Keluar");
        println!("Enter your choice:");

        let pilihan: String = get_user_input("Enter your choice");
        match pilihan.trim().parse::<u32>() {
            Ok(1) => tambah_barang_terjual(&mut daftar_barang_terjual),
            Ok(2) => tampilkan_barang_terjual(&daftar_barang_terjual),
            Ok(3) => edit_barang_terjual(&mut daftar_barang_terjual),
            Ok(4) => hapus_barang_terjual(&mut daftar_barang_terjual),
            Ok(5) => {
                println!("Keluar dari program. Selamat tinggal!");
                break;
            }
            _ => println!("Pilihan tidak valid. Silahkan coba lagi."),
        }
    }
}
