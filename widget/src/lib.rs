wit_bindgen::generate!({
    path: "target/wit",
    world: "process-v0",
});

kinode_process_lib::widget!("Tickers", create_widget);

fn create_widget() -> String {
    return r#"<html>

    <head>
        <meta name="viewport" content="width=device-width, initial-scale=1">
        <link rel="stylesheet" href="/kinode.css">
        <style>
        * {{
            box-sizing: border-box;
            margin: 0;
            padding: 0;
            font-family: 'Kode Mono', monospace;
        }}

        body {{
            scrollbar-color: transparent transparent;
            scrollbar-width: none;
        }}
        </style>
    </head>

    <body style="margin: 0; width: 100%; height: 100%; background: transparent;">
        <div id="table" style="margin: 0; width: 100%; height: 100%;"></div>
        <script>
            fetch('https://api.coincap.io/v2/assets?limit=10')
                .then(response => response.json())
                .then(data => {
                    const tableContainer = document.createElement('div');
                    tableContainer.style.cssText = 'width: 100%; height: 100%; overflow: auto; font-family: "Kode Mono", monospace;';

                    const table = document.createElement('table');
                    table.style.cssText = 'width: 100%; border-collapse: collapse;';

                    const headerRow = table.insertRow();
                    ['token', 'market cap (USD)', 'price (USD)'].forEach(text => {
                        const th = document.createElement('th');
                        th.textContent = text;
                        th.style.cssText = 'padding: 12px; text-align: left;';
                        headerRow.appendChild(th);
                    });

                    data.data.forEach(coin => {
                        const row = table.insertRow();
                        [
                            coin.symbol,
                            (parseFloat(coin.marketCapUsd) / 1e9).toFixed(2) + 'B',
                            parseFloat(coin.priceUsd).toLocaleString('en-US', {style: 'currency', currency: 'USD'})
                        ].forEach(text => {
                            const cell = row.insertCell();
                            cell.textContent = text;
                            cell.style.cssText = 'padding: 6px; border-bottom: 1px solid #ddd;';
                        });
                    });

                    tableContainer.appendChild(table);
                    document.getElementById('table').appendChild(tableContainer);
                })
                .catch(error => console.error('Error:', error));
        </script>
    </body>

    </html>"#
        .to_string();
}
