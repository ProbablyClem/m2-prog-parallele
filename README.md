# m2-prog-parallele
## Goupe
- GUITON Clément
- CHEMIN Bastien
- AZOUZ Theo
- AOUN Yorgo

## Technologies
Nous utilisons Rust car c'est un language de programmation très performant, permettant de faire de la programation concurrente facilement et sans risque de race condition.

Nous utilisons la librairie `rayon`, qui l'est l'outil Rust le plus populaire de parraléliser des taches.

## Cas 1
Nous avons choisis de lire un fichier de logs ligne par ligne, pour compter le nombre de lignes en erreur.

Pour effectuer notre test, nous avons implémenté deux fonctions qui parcourent le fichier de logs et qui comptent le nombre de lignes en erreur. La première fonction est séquentielle et la deuxième est parallèle.

Nous avons effectué 100 000 tests pour chaque fonction et nous avons calculé la moyenne des temps d'éxécution. Afin d'éviter les effets de bords, nous avons effectué un test préliminaire pour chauffer la machine.

### Résultats
### Fichier de 4.90MB et 56 000 lignes
- Time elapsed in warmup is: 4.957µs
- Mean time elapsed in read_file_serial() is: 4.497195ms
- Mean time elapsed in read_file_parallel() is: 24.23089ms

### Fichier de 1.47GB et 11 millions de lignes
- Time elapsed in warmup is: 3.991µs
- Mean time elapsed in read_file_serial() is: 1.159262837s
- Mean time elapsed in read_file_parallel() is: 5.308530566s

### Analyse 
Nous constatons ici que la fonction séquentielle est plus rapide que la fonction parallèle.
Cela s'explique par le fait que chaque unité de travail est très petite et que le coût de la parallélisation est plus grand que le coût de la séquentialisation.
De plus, la lecture d'un fichier est une opération séquentielle, donc la parallélisation n'apporte pas de gain de performance.

## Cas 2
Pour le deuxième cas, nous gardons le même principe que le premier cas, mais la lecture du fichier se fait toujours de séquentiel, tandis que seul le traitement des lignes est parallèle.

### Résultats
### Fichier de 4.90MB et 56 000 lignes
- Time elapsed in warmup is: 7.543425ms
- Mean time elapsed in read_file_serial() is: 6.134872ms
- Mean time elapsed in read_file_parallel() is: 6.365241ms

### Fichier de 1.47GB et 11 millions de lignes
- Time elapsed in warmup is: 2.123232429s
- Mean time elapsed in read_file_serial() is: 1.692442484s
- Mean time elapsed in read_file_parallel() is: 1.863808356s

### Analyse 
Nous constatons ici que la différence de temps d'éxécution entre la fonction séquentielle et la fonction parallèle est très faible.
En effet, la majorité du temps d'éxécution est consacré à la lecture du fichier, qui est une opération séquentielle.

## Cas 3
Nous prenons la meme logique que le cas 2, mais cette fois, nous ajoutons un .to_lowercase() sur chaque ligne.
Ce qui rend le traitement de chaque ligne plus long.

### Résultats
### Fichier de 4.90MB et 56 000 lignes
- Time elapsed in warmup is: 7.529008ms
- Mean time elapsed in read_file_serial() is: 11.660949ms
- Mean time elapsed in read_file_parallel() is: 7.733803ms

### Fichier de 1.47GB et 11 millions de lignes
- Time elapsed in warmup is: 2.031219287s
- Mean time elapsed in read_file_serial() is: 3.080727283s
- Mean time elapsed in read_file_parallel() is: 1.77909305s

### Analyse 
Nous constatons ici une nette amélioration de la fonction parallèle par rapport à la fonction séquentielle.
En effet, chaque unité de travail etant plus longue, le coût de la parallélisation est plus faible que le coût de la séquentialisation.

## Conclusion
La parralélisation est un outil efficace qui peut grandement améliorer les performances d'un programme.
Cependant, ce n'est pas une solution miracle a tous les problèmes.
la parralélisation demande une certaine réflexion et une bonne connaissance du problème à résoudre pour être efficace.
Elle peut meme avoir un impact négatif sur les performances si elle est mal utilisée.
