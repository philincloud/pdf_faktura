use printpdf::*;
use serde::Deserialize;
use std::{
    env,
    fs::File,
    io::{BufReader, BufWriter},
};

#[derive(Deserialize)]
struct Invoice {
    // logo_b64: Option<String>,  // disabled for now to make it compile
    issue_date: String,
    invoice_number: String,
    seller: Contractor,
    buyer: Contractor,
    items: Vec<Item>,
    total_gross: String,
    paid: String,
    payment_method: String,
    due_date: String,
    bank_name: String,
    account_number: String,
    issuer: String,
    footer: String,
}

#[derive(Deserialize)]
struct Contractor {
    name: String,
    nip: String,
    address_1: String,
    address_2: String,
}

#[derive(Deserialize)]
struct Item {
    index: String,
    name: String,
    unit: String,
    qty: String,
    net_price: String,
    vat_rate: String,
    net_val: String,
    vat_val: String,
    gross_val: String,
}

fn wrap_text(text: &str, max_chars: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();
    for word in text.split_whitespace() {
        if current_line.chars().count() + word.chars().count() + 1 > max_chars {
            if !current_line.is_empty() {
                lines.push(current_line.trim().to_string());
            }
            current_line = String::new();
        }
        current_line.push_str(word);
        current_line.push(' ');
    }
    if !current_line.is_empty() {
        lines.push(current_line.trim().to_string());
    }
    lines
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <input_json> <output_pdf>", args[0]);
        std::process::exit(1);
    }

    let input_path = &args[1];
    let output_path = &args[2];

    let file = File::open(input_path).expect("Failed to open input JSON");
    let reader = BufReader::new(file);
    let data: Invoice = serde_json::from_reader(reader).expect("Failed to parse JSON");

    let (doc, page1, layer1) = PdfDocument::new("Faktura", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    let font = doc
        .add_external_font(
            File::open("assets/Roboto-Regular.ttf")
                .expect("Put Roboto-Regular.ttf in assets/ folder"),
        )
        .expect("Failed to load font");

    let write_text = |text: &str, size: f32, x: f32, y: f32| {
        current_layer.use_text(text, size, Mm(x), Mm(y), &font);
    };

    write_text("Data wystawienia:", 10.0, 130.0, 280.0);
    write_text(&data.issue_date, 10.0, 170.0, 280.0);
    write_text(
        &format!("FAKTURA FV {}", data.invoice_number),
        14.0,
        85.0,
        255.0,
    );

    write_text("Sprzedawca", 10.0, 15.0, 237.0);
    write_text("Nabywca", 10.0, 110.0, 237.0);

    write_text(&data.seller.name, 9.0, 15.0, 230.0);
    write_text(&format!("NIP: {}", data.seller.nip), 9.0, 15.0, 225.0);
    write_text(&data.seller.address_1, 9.0, 15.0, 220.0);
    write_text(&data.seller.address_2, 9.0, 15.0, 215.0);

    write_text(&data.buyer.name, 9.0, 110.0, 230.0);
    write_text(&format!("NIP: {}", data.buyer.nip), 9.0, 110.0, 225.0);
    write_text(&data.buyer.address_1, 9.0, 110.0, 220.0);
    write_text(&data.buyer.address_2, 9.0, 110.0, 215.0);

    let mut y = 180.0f32;

    write_text("Lp.", 8.0, 15.0, y);
    write_text("Nazwa towaru lub usługi", 8.0, 25.0, y);
    write_text("J.m.", 8.0, 85.0, y);
    write_text("Ilość", 8.0, 95.0, y);
    write_text("Cena netto", 8.0, 110.0, y);
    write_text("VAT", 8.0, 130.0, y);
    write_text("Wartość netto", 8.0, 145.0, y);
    write_text("Wartość VAT", 8.0, 165.0, y);
    write_text("Wartość brutto", 8.0, 180.0, y);
    y -= 7.0;

    for item in &data.items {
        let wrapped_name = wrap_text(&item.name, 30);
        let row_height = (wrapped_name.len() as f32) * 5.0;

        write_text(&item.index, 8.0, 15.0, y);

        let mut text_y = y;
        for line in &wrapped_name {
            write_text(line, 8.0, 25.0, text_y);
            text_y -= 5.0;
        }

        write_text(&item.unit, 8.0, 85.0, y);
        write_text(&item.qty, 8.0, 95.0, y);
        write_text(&item.net_price, 8.0, 110.0, y);
        write_text(&item.vat_rate, 8.0, 130.0, y);
        write_text(&item.net_val, 8.0, 145.0, y);
        write_text(&item.vat_val, 8.0, 165.0, y);
        write_text(&item.gross_val, 8.0, 180.0, y);

        y -= row_height;
    }

    y -= 15.0;
    write_text("Razem:", 10.0, 15.0, y);
    write_text(&data.total_gross, 10.0, 45.0, y);

    y -= 10.0;
    write_text("Do zapłaty", 10.0, 15.0, y);
    write_text(&data.total_gross, 10.0, 45.0, y);

    y -= 5.0;
    write_text("Zapłacono:", 10.0, 15.0, y);
    write_text(&data.paid, 10.0, 45.0, y);

    write_text("Forma płatności:", 9.0, 15.0, 80.0);
    write_text(&data.payment_method, 9.0, 45.0, 80.0);
    write_text("Termin płatności:", 9.0, 15.0, 75.0);
    write_text(&data.due_date, 9.0, 45.0, 75.0);
    write_text("Bank:", 9.0, 15.0, 70.0);
    write_text(&data.bank_name, 9.0, 45.0, 70.0);
    write_text("Numer konta:", 9.0, 15.0, 65.0);
    write_text(&data.account_number, 9.0, 45.0, 65.0);

    write_text(&data.issuer, 10.0, 30.0, 40.0);
    write_text("Osoba upoważniona do wystawienia", 8.0, 15.0, 35.0);
    write_text("Osoba upoważniona do odbioru", 8.0, 120.0, 35.0);

    write_text(&data.footer, 6.0, 15.0, 15.0);

    doc.save(&mut BufWriter::new(
        File::create(output_path).expect("Failed to create PDF"),
    ))
    .expect("Failed to save PDF");

    println!("PDF generated successfully: {}", output_path);
}
