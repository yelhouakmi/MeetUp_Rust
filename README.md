# Meet-up Rust/Java

Ce projet à pour but d’accompagner le Meet-Up Rust/Java. Il contient l’implementation en Java et Rust d’un micro-service de demonstration.

## Endpoint

### Get /hello

Ce endpoint n’attends aucune donnée en entrée et renvoi « Hello, World! »

### Post /token/login

Simule un login avec retour d’un token d’authentification.
Ce endpoint attends un JSON au format:

```json
{
    "username": "name",
    "password": "password"
}
```

Si le mot de passe est «test», le service valide le login, génère un token (un UUID) et le renvoi ainsi que le username:

```json
{
    "token":"1bfdfa7e-afc7-4657-b092-46b21ffcbe8f",
    "user":"name"
}
```

Pour tout autre mot de passe, le service renvoi une erreur 403.

### Get /token/check/{token}

Simule une validation de token d’accès.
Le service attends un token (renvoyé par le service de login) en paramètre d’url.

Le token est cherché en cache et renvoyé ainsi que l’utilisateur concerné si disponible:

```json
{
    "token":"1bfdfa7e-afc7-4657-b092-46b21ffcbe8f",
    "user":"name"
}
```

Si le token n’existe pas, une erreur 401 est renvoyée.

## Compilation, test, run des projets

Ce projet contient une configuration GitPod. Le moyen le plus simple de tester les deux services est de les lancer directement dans GitPod. Les toolchain Java et Rust seront configurées au lancement du pod.

### Rust

Afin de faire fonctionner le projet Rust, la toolchain Rust doit être disponible dans le Path, voir [installation](https://www.rust-lang.org/learn/get-started).

La compilation en Rust peut être faite selon plusieurs profils. Les plus important pour ce projet sont:

- Le profil Release -> Permet une compilation plus rapide, au détriment de l’optimisation du binaire. C’est le mode par defaut de cargo:

```Shell
cargo <command>
```

- Le profil Release -> Ce profil propose le plus d’optimisation du code et est donc plus long à compiler. Tous les bench doivent être fait avec ce profil afin de représenter le comportement attendu de l’application en Prod:

```Shell
cargo <command> --release
```

Le paramètre de commande «--release» peut être ajouté à toutes les commandes cargo impliquant une compilation (build, test, run).

#### Compilation

La compilation du projet passe par la commande `cargo build`.

Toutes les dependences du projet sont téléchargées si elles ne sont pas en cache puis compilées avant le compilé le projet en lui même. Une phase d’optimisation est ensuite lancée (Tree shaking, inlining,…) avant la génération du binaire.

La compilation des différentes deps est faite en parallèle en fonction des différences interdependances.

#### Test

Les tests ne sont pas lancées automatiquement lors de la compilation d’un projet, ils doivent être lancées manuellement via la commande `cargo test`.

Le projet est alors compilé si besoin, les tests sont ensuite compilés et lancées. Un rapport est affiché dans le terminal.

#### Run

Le lancement du projet se fait via la commande `cargo run`. Le lancement du projet déclenche l’étape de build mais pas l’étape de test.

### Java

Le projet Java utilise Maven, il doit donc être disponible dans le Path.
Afin de compiler l’image native, la JVM doit être Une édition de GraalVM. Les test ont été fait avec Mandrel qui est une GraalVM donc tous les élément ne concernant pas Java ont été retirés. Mandrel est supporté par RedHat et à été créée pour soutenir la compilation Native Quarkus.
Le compilation native peut aussi se faire via un conteneur docker directement via Maven mais ce cas n’a pas été utilisé.

#### Compilation

La compilation JVM se fait via la commande `mvn compile`. Les dependences sont téléchargées si elles ne sont pas en cache puis le projet est compilé.

Pour obtenir un binaire natif, il faut passer par la commande `mvn package -Pnative`. Le paramètre «-Pnative» active le profil de compilation natif. La compilation native déclenche le lancement des tests.

#### Tests

Les tests JVM/Natif entrent en conflit et pose problème pour le lancement des tests. La documentation Quarkus n’étant pas clair sur la démarche à adopter.

Les tests JVM sont lancées via la commande `mvn test`. Ces tests se lancent sur la version JVM de l’application. Pour lancer les tests sur la version native, le binaire doit avoir été précédement créé.

#### Run

Afin de lancer la version JVM, il faut indiquer le Jar de lancement Quarkus:

```Shell
    java -jar target/quarkus-app/quarkus-run.jar
```

Le binaire peut être directement executé dans le cadre de la version native:

```Shell
    ./target/java-meet-up-1.0.0-SNAPSHOT-runner
```
