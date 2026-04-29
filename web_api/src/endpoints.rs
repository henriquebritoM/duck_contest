use axum::{Error, response::IntoResponse};

use crate::structs::duck;

//  CRUD
//  Create
//  Read
//  Update
//  Delete

// 1. Criar um Pato (POST /patos)
// Cria um novo registro de pato no lago.
//  * Payload (Exemplo):
//    {
//   "apelido": "Zeca",
//   "raca": "Pato-real",
//   "habilidades": ["Nadar", "Grasnar alto"]
// }

//  * Regras:
//    * apelido: Obrigatório, único (não podem existir dois patos com o mesmo apelido), string de até 32 caracteres.
//    * raca: Obrigatório, string de até 32 caracteres.
//    * habilidades: Opcional, array de strings.
//  * Respostas Esperadas:
//    * 201 Created - Se o pato for criado com sucesso (deve retornar o cabeçalho Location: /patos/{id}).
//    * 422 Unprocessable Entity - Se faltar algum campo obrigatório ou o formato estiver incorreto.
//    * 409 Conflict - Se já existir um pato com esse apelido.


//  Notas: Ver como pegar o conteúdo do body - ver como pegar o conteúdo do 
//   Path. Documentação da crates.io é o bastante
pub async fn add_user() -> impl IntoResponse {

    let user_json = 
}

// 2. Buscar um Pato por ID (GET /patos/[:id])
// Retorna os detalhes de um pato específico.
//  * Respostas Esperadas:
//    * 200 OK - Com o JSON do pato no corpo da resposta.
//    * 404 Not Found - Se o ID não existir no banco.
pub async fn search_user(user: duck) -> Result<(), Error> {

    todo!()
}

// 3. Busca Geral (GET /patos?t=[:termo])
// Busca patos cujo apelido ou raca contenham o termo passado na query string.
//  * Exemplo: GET /patos?t=real
//  * Respostas Esperadas:
//    * 200 OK - Com um array de patos (pode ser vazio [] se não achar nada).
//    * 400 Bad Request - Se o parâmetro t não for enviado ou for vazio.
pub async fn search_users(user: duck) -> Result<(), Error> {

    todo!()
}

// 4. Contagem de Patos (GET /contagem-patos)
// Um endpoint simples para sabermos quantos patos sobreviveram no banco de dados.
//  * Respostas Esperadas:
//    * 200 OK - Retornando apenas o número inteiro em texto puro (ex: 42).
pub async fn count_users(user: duck) -> Result<(), Error> {

    todo!()
}

pub async fn del_user(user: duck) -> Result<(), Error> {

    todo!()
}

pub async fn update_user(user: duck) -> Result<(), Error> {

    todo!()
}