# GitHub Actions Trusted Publishing Setup

Este projeto usa **Trusted Publishing** para publicar automaticamente no crates.io, que é mais seguro que usar tokens!

## ✅ Configurar Trusted Publisher no crates.io:

1. Acesse: https://crates.io/settings/tokens
2. Role até "Trusted Publishers"
3. Clique em "Add a new Trusted Publisher"
4. Preencha:
   - **Publisher**: `GitHub`
   - **Repository owner**: `marcus-gomes-v`
   - **Repository name**: `wallet-signature-verify`
   - **Workflow filename**: `release.yml`
   - **Environment name**: deixe vazio (opcional)
5. Clique em "Add"

**Pronto!** Não precisa de nenhum secret ou token! 🎉

## 🚀 Como usar:

Depois de configurar o Trusted Publisher, o workflow automático irá:

1. **Quando você criar uma tag** (ex: `v0.2.1`):
   - ✅ Roda todos os testes
   - ✅ Publica automaticamente no crates.io (sem token!)
   - ✅ Cria uma GitHub Release
   - ✅ Gera changelog automático

## 📝 Exemplo de uso:

```bash
# 1. Atualize a versão no Cargo.toml
# version = "0.2.1"

# 2. Commit as mudanças
git add .
git commit -m "chore: bump version to 0.2.1"
git push

# 3. Crie a tag
git tag -a v0.2.1 -m "Release v0.2.1"
git push origin v0.2.1

# 4. O GitHub Actions faz o resto automaticamente! 🚀
```

## 🔍 Verificar se funcionou:

- GitHub Actions: https://github.com/marcus-gomes-v/wallet-signature-verify/actions
- Crates.io: https://crates.io/crates/wallet-signature-verify
- Releases: https://github.com/marcus-gomes-v/wallet-signature-verify/releases

## 🔐 Por que Trusted Publishing é melhor?

- ✅ Sem tokens para gerenciar
- ✅ Sem secrets para configurar
- ✅ Mais seguro (usa OIDC)
- ✅ Revogação automática ao remover do crates.io
- ✅ Logs de auditoria no crates.io
