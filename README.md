# m2-prog-parallele
## Goupe
- GUITON Clément
- CHEMIN Bastien
- AZOUZ Théo
- AOUN Yorgo

## Technologies
Nous utilisons Rust car c'est un language de programmation tres performant permettant de faire de la programation concurrente facilement et sans risque de race condition.

Nous utilisons la librairie `rayon` pour faire de la programmation parallèle.

## Cas 1
Nous avons choisis de lire un fichier de logs ligne par ligne, pour compter le nombre de lignes en erreur.

Pour éffectuer notre test, nous avons implémenté deux fonctions qui parcourent le fichier de logs et qui comptent le nombre de lignes en erreur. La première fonction est séquentielle et la deuxième est parallèle.
La fonction séquentielle parcourt le fichier de logs ligne par ligne et incrémente un compteur si la ligne contient le mot "error".
La fonction parallèle lit le fichier de manière sequentielle également, mais s'occuper de filter les lignes en erreur en parallèle.

Nous avons effectué 100 000 tests pour chaque fonction et nous avons calculé la moyenne des temps d'éxécution. Afin d'éviter les effets de bords, nous avons effectué un test préliminaire pour chauffer la machine.

### Résultats
### Fichier de 4.90MB et 56 000 lignes
Time elapsed in warmup is: 4.957µs
- Mean time elapsed in read_file_serial() is: 4.497195ms
- Mean time elapsed in read_file_parallel() is: 24.23089ms

### Fichier de 1.47GB et 11 millions de lignes
Time elapsed warmup is: 3.991µs
Mean time elapsed in read_file_serial() is: 1.159262837s
Mean time elapsed in read_file_parallel() is: 5.308530566s

### Conclusion
Nous constatons ici que la fonction séquentielle est plus rapide que la fonction parallèle.
Cela s'explique par le fait que chaque unité de travail est très petite et que le coût de la parallélisation est plus grand que le coût de la séquentialisation.