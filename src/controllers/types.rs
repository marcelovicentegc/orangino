use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: u32,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Employee {
    pub id: u32,
    pub nome: String,
    pub usuario: User,
    pub selectedDataInicio: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct CheckUserResp {
    pub status: String,
    pub funcionario: Employee,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PunchResp {
    pub allowAll: bool,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SyncResp {
    // Possible values are: NOVO_PONTO_ABERTO,
    // ULTIMO_PONTO_FECHADO_NOVO_ABERTO and
    // NEGADO_FORA_HORARIO_PERMITIDO (would an
    // enum fit here?)
    pub tipoRetornoRegistroApontamentoEnum: String,
    pub mensagem: String,
    pub statusPonto: i32,
    pub sucesso: bool,
    pub excluirPonto: bool,
}
