# ENTLite

Un client intelligent pour l'ENT.

## Pour-quoi faire ?

Si vous êtes dans un lycée public en Ile-de-France, vous avez sûrement déjà eu affaire au terrible [ENT](https://monlycee.net/).
Le client officiel ENT est construit sur AngularJS, dont le [support a officiellement pris fin en Janvier 2022](https://angularjs.org/). De fait, ENT est lent et vulnérable.
Je propose donc une implémentation open-source d'un client ENT, basée sur des technologies modernes.

## Comment ça marche ?

J'ai inspecté les requêtes envoyées aux serveurs de l'ENT à partir du client officiel pour construire celui-ci.
Comme la politique CORS des serveurs de l'ENT n'acceptent pas les requêtes cross-origin, ce client inclut un proxy qui fait les requêtes à l'ENT.

## Comment l'utiliser ?

```sh
git clone https://github.com/JanotLeLapin/ENTLite
cd ENTLite

# Avec Docker
docker build . -t entlite
docker run -p3000:3000 entlite

# Sans Docker (prérequis: Node, PNPM)
pnpm build
pnpm start
```

Vous pouvez désormais ouvrir `http://localhost:3000` sur votre navigateur préféré.