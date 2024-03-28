use chrono::prelude::Utc;
use std::fs::File;
use std::io::Write;

pub struct Html {}

impl Html {
    pub fn wrap_with_skeleton(s: &str) -> String {
        format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Plise Measurements</title>
    <style>
        table {{
            width: 100%;
            border-collapse: collapse;
        }}
        th, td {{
            border: 1px solid black;
            padding: 8px;
            text-align: left;
        }}
        th {{
            background-color: #f2f2f2;
        }}
    </style>
</head>
<body>
{}
</body>
</html>"#,
            s
        )
    }

    pub fn create_consumables_html(s: &str, maliyet: f32, client: &str) {
        let date = Utc::now();
        let table = format!(
            r#"
<table>
    <tr>
        <th colspan="12", style="text-align: center;">DOĞUŞ SİNEKLİK SARF MALZEME TABLOSU</th>
    </tr>
    <tr>
        <th style="border-right: none;">Müşteri Adı:</th>
        <th style="border-left: none;">{}</th>
        <th colspan="8"></th>
        <th style="border-right: none;">Tarih:</th>
        <th style="border-left: none;">{}</th>
    </tr>
    <tr>
        <th>Sıra</th>
        <th>Plise Tipi</th>
        <th>Renk Tipi</th>
        <th>Kasa (cm)</th>
        <th>Kanat (cm)</th>
        <th>Tül (cm^2)</th>
        <th>Şerit (cm)</th>
        <th>Köşe Adet</th>
        <th>Teker Adet</th>
        <th>Klips Adet</th>
        <th>Stop Adet</th>
        <th>Dönüş Adet</th>
    </tr>
{}
<tr>
<td colspan="10"></td>
<td style="border-right: none;">Toplam Maliyet Fiyatı:</td>
<td style="border-left: none;">{:.2} TL</td>
</tr>
</table>
"#,
            client,
            date.format("%d.%m.%Y"),
            s,
            maliyet
        );

        let content = Html::wrap_with_skeleton(&table);
        let mut file = File::create("maliyet.html").unwrap();
        file.write_all(content.as_bytes()).unwrap();
        webbrowser::open("maliyet.html").unwrap();
    }

    pub fn create_price_html(s: &str, price: f32, kdv: f32, client: &str) {
        let date = Utc::now();
        let table = format!(
            r#"
<h2>Plise Fiyat Tablosu</h2>
<table>
    <tr>
        <th colspan="7", style="text-align: center;">DOĞUŞ SİNEKLİK ÜCRET TABLOSU</th>
    </tr>
    <tr>
        <th>Müşteri Adı: {}</th>
        <th colspan="2"></th>
        <th style="text-align: right;">Tarih: {}</th>
    </tr>
    <tr>
        <th>Sıra</th>
        <th>Plise Ölçüsü</th>
        <th>Plise Tipi</th>
        <th>Boya Tipi</th>
    </tr>
{}
<tr>
<td style="border-bottom: none;" colspan="3"></td>
<td>Toplam Fiyat: {:.2} TL</td>
</tr>
<tr>
<td style="border-bottom: none; border-top: none;" colspan="3"></td>
<td>KDV (%) {}</td>
</tr>
<tr>
<td style="border-top: none;" colspan="3"></td>
<td>Kdv Dahil Fiyat: {:.2}</td>
</tr>
</table>
"#,
            client,
            date.format("%d.%m.%Y"),
            s,
            price, kdv, price * (1.0 + kdv / 100.0)
        );

        let content = Html::wrap_with_skeleton(&table);
        let mut file = File::create("fiyat.html").unwrap();
        file.write_all(content.as_bytes()).unwrap();

        webbrowser::open("fiyat.html").unwrap();
    }
}
