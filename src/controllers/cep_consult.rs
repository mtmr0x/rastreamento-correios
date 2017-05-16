pub mod cep_consult_controller {
    extern crate rustc_serialize;

    use std::collections::BTreeMap;
    use rustc_serialize::json::{Json, ToJson};

    #[derive(RustcDecodable, RustcEncodable)]
    struct CEPResponse {
      cep: String,
      tipo_de_logradouro: String,
      logradouro: String,
      bairro: String,
      cidade: String,
      estado: String,
    }

    impl ToJson for CEPResponse {
        fn to_json(&self) -> Json {
            let mut map = BTreeMap::new();
            map.insert("cep".to_string(), self.cep.to_json());
            map.insert("tipo_de_logradouro".to_string(), self.tipo_de_logradouro.to_json());
            map.insert("logradouro".to_string(), self.logradouro.to_json());
            map.insert("bairro".to_string(), self.bairro.to_json());
            map.insert("cidade".to_string(), self.cidade.to_json());
            map.insert("estado".to_string(), self.estado.to_json());
            Json::Object(map)
        }
    }

    pub fn receive_cep(cep: String) {
        let cep_number = CEPResponse {
            cep: cep.to_string(),
            tipo_de_logradouro: "".to_string(),
            logradouro: "".to_string(),
            bairro: "".to_string(),
            cidade: "".to_string(),
            estado: "".to_string(),
        };

        println!("vish {}", cep_number.to_json());

        cep_number.to_json();
    }
}

