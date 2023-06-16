
# Tasker

## Objectif

Au cours de cette session, on se lance dans la création d'un gestionnaire de taches personnel:
**Tasker**.

Le binaire est un utilitaire en ligne de commande qui nous permettra de sauvegarder ou lister nos tâches en l'appellant.
On souhaite également qu'il nous permettre de gérer des priorités. D'autres fonctionnalités viendront s'ajouter au fur et à mesure du développement.

## Specification #1: Ajouter une tâche en ligne de commande

### Structure Task

On commance par développer la capacité d'instancier une tâche.

Via une commande add: `tasker add "<label>"`
Exemple: `tasker add "Acheter du lait"`

<details>
<summary>Rappel de la syntaxe pour définir une structure (+)</summary>

```rust
struct StructName {
    field_one: Type,
    field_two: Type,
} 
```

</details>

 - Créer un nouveau projet `tasker` en utilisant `cargo new <project>` ([documentation](https://doc.rust-lang.org/cargo/commands/cargo-new.html))

 - Dans `main`, définir une structure `Task` qui dispose d'un champ `desc` de type `String`.

 - Créer un block `impl` avec une méthode statique `new` qui prend un paramètre `String` et qui retourne `Self` pour faire office de constructeur.

 - Ajouter un module de tests sous la fonction main avec un test du constructeur puis tester avec la commande cargo adaptée.

    <details>
    <summary>Afficher le code pour ajouter un module de test (+)</summary>

    ```rust
    // compilation uniquement pour les tests
    #[cfg(test)]
    mod tests { // scope à part, les imports de main ne sont pas visibles ici
        use super::*;
        #[test]
        fn constructor_works_and_sets_description() {
            assert_eq!(Task::new("Hello".to_string()).desc, "Hello");
        }
    }
    ```

    </details>

 - Ajouter à `fn main()` le contenu suivant pour simuler un argument en ligne de commande puis vérifier le fonctionnement du code en lancant le programme.

    ```rust
    let fake_task_argument = String::from("faire un café ☕");
    let task = Task::new(fake_task_argument);
    dbg!(println!("task description: {}", task.desc));
    ```

### Et la récupération des arguments alors ?


<details>
    <summary>On se contente d'ajouter le code suivant ci dessous (+)</summary>

```rust
    use structopt::StructOpt;

    #[derive(Debug, StructOpt)]
    #[structopt(name = "tasker", about = "Manage your tasks")]
    struct Args {
        #[structopt(subcommand)]
        cmd: Cmd,
    }

    #[derive(Debug, StructOpt)]
    enum Cmd {
        Add(Desc),
    }

    #[derive(Debug, StructOpt)]
    struct Desc {
        desc: String,
    }

    let Args { cmd: Cmd::Add(Desc{desc}) } = Args::from_args();
```

</details>

Il utilise la librairie [structopt](https://docs.rs/structopt/latest/structopt/#).
On recupera plus tard nous meme les arguments en apprenant d'autres concepts.

Pour l'instant, il est possible de présenter ce code et sa syntaxe, puisqu'il utilise:

* un import,
* des structures,
* une enum,
* des attributs,
* du destructuring.

- Supprimer la variable `fake_task_argument` et utiliser `desc` à sa place dans le constructeur de Task.


## Specification #2: Ajouter une priorité à une tache

**Point cours énumérations**

<details>
<summary>Rappel de la syntace pour definir une enum</summary>

```rust
enum EnumName {
    VariantOne,
    VariantTwo,
} 
```

</details>

 -  Definir une `enum` `Priority` qui dispose de trois variants: `Low`, `Normal`, `High`

 -  Ajouter un champ `priority` à la structure `Task`

 -  Mettre à jour le contructeur de `Task` (`fn new (desc: String, prio: Priority) -> Self { ... }`)

 -  Ajouter si vous le souhaitez une directive `#[allow(dead_code)]`

 -  Ajouter le test unitaire de validation de la construction d'une tâche avec une priorité (noter l'utilisation de `assert_eq!`)

<details>
<summary>Test unitaire de validation du contructeur avec une priorité haute</summary>

```rust
#[test]
    fn build_task_with_label_and_priority_high_priority() {
        assert_eq!(
            Task::new(String::from(""), Priority::High, Deadline::None).prio,
            Priority::High
        );
    }
```

</details>

## Specification #3: Spécifier une dealine

<details>
<summary>Rappel de la syntaxe pour un variant qui dispose de contenu</summary>

```rust
enum EnumName {
    VariantOne(String),
    VariantTwo(u64, Task),
    VariantStructLike{ field: String },
    VariantStructLike{ text: String, task: Task },
}
```

</details>

 -  Definir une `enum` `Deadline` qui dispose de 2 variants: `None`, `Date(chrono::DateTime<Utc>)`

 -  Ajouter la librairie `chrono` en tant que dépendance dans le Cargo.toml

 -  Définir un alias pour simplifier la lecture et l'écriture du code: `type Date = chrono::DateTime<Utc>;`

 -  Ajouter le test de construction d'une `Task` avec une deadline (noter l'utilisation de `assert_ne!`)

```rust
#[test]
       fn task_deadline_is_NOT_none_when_set_with_date() {
           assert_ne!(
                   Task::new(String::from(""), Priority::High, Deadline::Date(chrono::Utc::now())).due,
                   Deadline::None
                   );
       }
```

## Specification #4: Afficher une tache avec son urgence

On veut afficher une tâche sur une ligne avec sa priorité et sa deadline au format: 

 - `(!) label de la tache` si la tâche est urgente,
 - `--- label de la tache` si la tâche n'est pas urgente.

Une tâche est urgente si elle est à réaliser dans les 24h ou si elle est prioritaire.

 -  Créer une méthode `fn is_urgent(self) -> bool`:

    <details>
    <summary>template de la méthode is_urgent</summary>

    ```rust
        impl Task {
            fn is_urgent(self) -> bool {
                unimplemented!()
            }
        }
    ```

    </details>

 -  Ajouter les tests unitaires (à la suite des précédents) sans oublier d'importer `use chrono::{prelude::*, Days};` à côté de `use super::*;` dans le module de tests.

    <details>
    <summary>Code des tests de task::is_urgent</summary>

    ```rust
    #[test]
    fn task_is_urgent_when_priority_is_high_whatever_the_deadline() {
    assert_eq!(
            Task::new("".into(), Priority::High, Deadline::None).is_urgent(),
            true
            );
    assert_eq!(
            Task::new("".into(), Priority::High, Deadline::Date(Utc::now())).is_urgent(),
            true
            );
    assert_eq!(
            Task::new(
                "".into(),
                Priority::High,
                Deadline::Date(Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap())
                )
            .is_urgent(),
            true
            );
    }

    #[test]
    fn task_is_urgent_when_date_is_next24h_whatever_the_priority() {
    assert_eq!(
            Task::new("".into(), Priority::Low, Deadline::Date(Utc::now())).is_urgent(),
            true
            );
    assert_eq!(
            Task::new(
                "".into(),
                Priority::High,
                Deadline::Date(Utc::now() + Days::new(1))
                )
            .is_urgent(),
            true
            );
    }

    #[test]
    fn task_is_not_urgent_when_date_is_not_next24h_and_priority_is_not_high() {
    assert_eq!(
            Task::new("".into(), Priority::Normal, Deadline::None).is_urgent(),
            false
            );
    assert_eq!(
            Task::new(
                "".into(),
                Priority::Low,
                Deadline::Date(Utc::now() + Days::new(3))
                )
            .is_urgent(),
            false
            );
    }
    ```

    </details>

 - Implémenter la méthode `is_urgent` (conseil: utiliser `match`, `self.due`, `+` `<` ou `>=`, `Utc::now()` `Days::new(1)`, `self.prio`...)

 - Ajouter la méthode `print_task` ci-dessous.

    <details>
    <summary>
    Code pour afficher une tache avec son urgence (ainsi que sa priorité et son échéance):
    </summary>

    ```rust
    fn print_task(task: Task) {
        let urgency_string = if task.is_urgent() { "(!)" } else { " - " };
        println!(
            // print!("{*>10}", valeur) signifie :
            // affiche la 'valeur' justifiée à droite ('>') sur 
            // au moins '10' colonnes et ajoute du padding
            // si necessaire avec des '*'"
            "{} {: <10} {: <40} {: <10} ",
            urgency_string,
            format!("{:?}", task.prio), // dirty trick pour afficher un type Debug mais pas display
            task.desc,
            format!("{:?}", task.due),
        );
    }
    ```

    </details>

> *Questions*:
> Peut-on appeler deux fois la méthode `is_urgent` ?
> Quelle est l'erreur du compilateur ?
> Que dire de la méthode `print_task` ?

 -  Transformer `print_task` en tant que methode de `Task` (transformer sa signature en `print(self)` tout en l'ajoutant au bloc `impl Task`)

**Point cours ownership**

On dispose d'au moins deux choix:
- **Choix 1**: Retourner la meme valeur `fn f(self) -> self` et la reassigner. Mais ca sous entend toujours récupérer la valeur. Ici c'est compliqué sachant qu'on attend un bool.
- **Choix 2**: Copier l'instance avant de la passer en paramètre.
- **Choix 3**: Cloner, similaire à copy mais explicite.
- **Choix 4**: Emprunter la valeur à la variable qui la possède. (on y revient dans un instant).

Pour rendre l'instance copiable, il suffit de deriver le trait Copy en annotant la structure.

**Point sur les [marqueurs](https://doc.rust-lang.org/std/marker/struct.PhantomData.html)**

 -  Essayer de le faire (`#[derive(Copy)]`).

    > *Questions*:
    > Quelles conséquences ?
    > Notamment pour les types qui composent notre structure ?
    > A noter: la copie est silencieuse ce que masque de l'information et peut mener à une mauvaise comprehension du code à la lecture.

 -  Deriver le trait Clone en annotant la structure pour essayer `#[derive(Clone)]` et appeler la méthode `clone()` lorsque nécessaire.

    **Point cours borrowing et références** (02_a + 05_poin)

 -  Modifier le code pour emprunter la valeur de `task` le temps des fonctions `print` et `is_urgent`.

    **Point cours borrowing et références mutables** <!-- https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html -->

## Un Builder pour nos tâches

La construction actuelle de notre structure est fastidieuse.
On aimerait pourvoir simplement spécifier le label de la tâche et mettre à jour priorité et échéance uniquement en cas de besoin,
et laisser ces valeurs par défaut autrement.
On peut mettre en place un builder (`TaskBuilder`) qui construit notre instance avec des valeurs par défaut.

### Priorité et échéance par défaut

La librairie standard Rust met à disposition un trait `Default` qui nous permet d'implémenter une méthode `default`.

 -  Trouver la documentation de ce trait.
 -  L'implémenter pour nos deux enums.

### Api [fluent](https://en.wikipedia.org/wiki/Fluent_interface)

Une Api fluent est une interface de programmation qui permet de chainer les appels de focntions ou de méthodes:

```rust
let task = TaskBuilder::new().description("bla bla").priority(Priority::High).deadline(D::None).build();
```

> Question:
> Quelle est la signature de chacune de ces methodes ?
> En particulier celle de build() ?

 -  Essayer d'implémenter ce builder.

    > Question:
    > Quelle difficulté(s) apparaî(t/ssent)?
    <details>
    <summary>
    Question cachée...
    </summary>

    > Comment gérer le caractère optionnel d'une valeur ?

    C'est le moment de faire un **Point sur les `Option`s** (cf. option_101 et [doc](https://doc.rust-lang.org/std/option/))

    > Comment travailler avec des références mutables ?

    </details>

    > Pourrait-on remplacer notre struct `Deadline` par un simple `Option<Date>` ? 

 -  Implémenter ce builder ! On remplacera notre deadline plus tard.

    <details>
    <summary>
    Tests unitaires pour vérifier le fonctionnement du builder
    </summary>

    On remarque que si aucune description n'est fournie, celle-ci devient une tâche sans label.

    ```rust
    #[test]
    fn builder_methodes_are_working() {
        let mut task = TaskBuilder::new()
            .description("".to_string())
            .priority(P::High)
            .deadline(D::None)
            .build();
        assert_eq!(task.desc, "".as_ref());
        assert_eq!(task.prio, P::High);
        assert_eq!(task.due, Deadline::None);
    }
    #[test]
    fn builder_by_default() {
        let mut task = TaskBuilder::new()
            .build();
        assert_eq!(task.desc, "".as_ref());
        assert_eq!(task.prio, P::Normal);
        assert_eq!(task.due, Deadline::None);
    }
    ```

    </details>

Le contruction d'une tâche sans description ne devrait pas être possible.

On pourrait même vérifier certaines contraintes (3 caractères minimun...) dans notre builder.

Pour indiquer que notre builder n'accepte pas les chaines de caractères vides ou nulles, on va retourner une enumération `BuildResult<Self, String>` depuis build.
On force ainsi à l'usage l'appelant à vérifier le résultat de son appel et à modifier sa construction de task si besoin.

 - Implémenter cette enumération et l'utiliser !

    > Question:<br>
    > Est ce fastidieux de créer un TypeResult à chaque fois qu'on voudra  gérer une erreur ?
    > On pourrait utiliser une exception ?

    **Point de cours `Result`**

 - Remplacer `BuildResult` par un `Result<Task, String>` et laisser l'erreur provoquer l'arret du programme pour afficher l'erreur (utiliser `?`).

Aller plus loin:

Les génériques sont pratiques dans le sens ou le code développé peut s'adapter à différents cas d'usage et nous éviter de nous répéter.

**Point sur les génériques**

Exemple d'usage:

```rust
fn discard_err<T, E>(result: Result<T, E>) -> Option<T> {
    match result {
        Ok(o) => Some(o),
        Err(_) => None,
    }
}

fn main() {
     println!("Get value 1!");

    let r: Result<char, isize> = Result::Ok('a');
    println!("{:?}", r);
    let r = discard_err(r);
    println!("{:?}", r);
    
    println!("Get value 2!");
    let r: Result<char, isize> = Result::Err(-1);
    println!("{:?}", r);
    let r = discard_err(r);
    println!("{:?}", r);
}
```

## "Parser" les composants de notre tâche

On souhaite laisser la possibilité à l'utilisateur d'ajouter une deadline en écrivant:

```rust
    tasker add "ma tache X due:24/01/2023 priority:Low"
```

Pour ce faire, on va devoir ajouter une méthode from_str pour chacune de nos structures.

- Ecrivez les signatures des methodes from_str de chaque type (utiliser `unimplemented()`)

- Ecriver les tests nécessaires.
  
- Implémentez les méthodes sachant que:
  - pour les dates on ne lit que le format 'DD/MM/YYYY',
  - pour les priorités, on ne lit que 'low, normal, high'

> Question:
> Comment gérer les erreurs ?
> Quelle différence entre panic, expect, `?` ?

## Un type associé

il existe un trait pour faire exactement ce qu'on vient de faire (https://doc.rust-lang.org/std/str/trait.FromStr.html).

**Point sur les traits**

**Point sur les types associés**

## Des tâches (en autonomie ou presque !)

On veut bien entendu travailler sur une liste de taches.

**Point sur les listes**

- On peut commencer par s'intéresser à récupérer les arguments (on supprime strctops) pour se faire la main: C'est parti ! [doc](https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html)

- Puis maintenant implémenter une liste de tâches.

Puis les sauvegarder en base avec sqlite (Avec rusqlite) !
