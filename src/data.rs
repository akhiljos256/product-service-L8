use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Google Pixel 8 Pro".to_string(),
            price: 899.99,
            description: "Capture stunning photos and experience pure Android with the Google Pixel 8 Pro, featuring AI-powered camera features and a smooth, adaptive display.".to_string(),
            image: "/google_pixel_8_pro.jpg".to_string(),
        },
        Product {
            id: 2,
            name: "Lenovo ThinkPad X1 Carbon".to_string(),
            price: 1399.99,
            description: "The Lenovo ThinkPad X1 Carbon is a lightweight yet powerful laptop, perfect for business professionals on the go.".to_string(),
            image: "/lenovo_thinkpad_x1_carbon.jpg".to_string(),
        },
        Product {
            id: 3,
            name: "JBL Flip 6 Bluetooth Speaker".to_string(),
            price: 129.99,
            description: "Take your music anywhere with the JBL Flip 6. This waterproof speaker delivers powerful bass and crystal-clear sound.".to_string(),
            image: "/jbl_flip_6.jpg".to_string(),
        },
        Product {
            id: 4,
            name: "ASUS ROG Strix Gaming Laptop".to_string(),
            price: 1999.99,
            description: "Dominate your games with the ASUS ROG Strix Gaming Laptop, equipped with the latest RTX graphics and high-refresh-rate display.".to_string(),
            image: "/asus_rog_strix.jpg".to_string(),
        },
        Product {
            id: 5,
            name: "Fitbit Charge 5 Fitness Tracker".to_string(),
            price: 149.99,
            description: "Track your health and fitness goals with the Fitbit Charge 5, offering heart rate monitoring, sleep tracking, and stress management tools.".to_string(),
            image: "/fitbit_charge_5.jpg".to_string(),
        },
        Product {
            id: 6,
            name: "LG C3 55-inch OLED TV".to_string(),
            price: 1799.99,
            description: "Immerse yourself in lifelike visuals with the LG C3 OLED TV, offering perfect blacks, vivid colors, and a sleek design.".to_string(),
            image: "/lg_c3_oled_tv.jpg".to_string(),
        },
        Product {
            id: 7,
            name: "HP Envy Photo 7855 Printer".to_string(),
            price: 199.99,
            description: "Print stunning photos and documents with the HP Envy Photo 7855, featuring wireless connectivity and an easy-to-use interface.".to_string(),
            image: "/hp_envy_7855.jpg".to_string(),
        },
        Product {
            id: 8,
            name: "Corsair K70 RGB Pro Keyboard".to_string(),
            price: 159.99,
            description: "Enhance your gaming setup with the Corsair K70 RGB Pro, featuring mechanical keys, customizable lighting, and durable construction.".to_string(),
            image: "/corsair_k70_rgb_pro.jpg".to_string(),
        },
        Product {
            id: 9,
            name: "Eufy RoboVac G30 Edge".to_string(),
            price: 279.99,
            description: "Keep your floors spotless with the Eufy RoboVac G30 Edge, a powerful robot vacuum with smart navigation.".to_string(),
            image: "/eufy_robovac_g30.jpg".to_string(),
        },
        Product {
            id: 10,
            name: "Canon EOS R50 Mirrorless Camera".to_string(),
            price: 899.99,
            description: "Capture your moments with precision using the Canon EOS R50, a compact mirrorless camera perfect for both beginners and enthusiasts.".to_string(),
            image: "/canon_eos_r50.jpg".to_string(),
        },
    ]
}
