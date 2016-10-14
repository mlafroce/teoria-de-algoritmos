# Trabajo Práctico n.º 1

## Estadístico de orden k

### Fuerza bruta

#### Orden

Este algoritmo itera secuencialmente por la lista aplicando un *validador* que compara un candidato contra todos los elementos de la lista. Este validador es de orden **O(n)** siempre.

En el caso *óptimo* el estadístico k se encuentra en el primer lugar, por lo que sólo se ejecuta una acción de orden **O(n)**, y el orden de este caso es **O(n)**

En el caso *pésimo* el estadístico k se encuentra en el último lugar, por lo que se ejecuta una acción de orden **O(n)** para cada elemento. El orden final en este caso es **O(n^2)**

El caso promedio el estadístico k se encuentra aproximadamente luego de evaluar n/2 candidatos, por lo que el orden final también es **O(n^2)**

#### Casos de ejemplo

K = 0

*Mejor caso*: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]

*Peor caso*: [16, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 1]

### Ordenar y seleccionar

#### Orden

Este algoritmo realiza una copia del vector original y la ordena con un ordenamiento rápido y estable de la biblioteca estandar de Rust (la descripción dice que el orden es *O(n.log(n))* en el peor caso, y utiliza aproximadamente 2n espacio en memoria, se sospecha que es merge-sort).
Con el vector ordenado, se selecciona el k-esimo elemento.

Si el algoritmo de ordenamiento es merge-sort, todos los casos tienen la misma cantidad de instrucciones. La complejidad de copiar el vector es O(n), ordenar el vector copiado es de O(n.log(n)), siempre, y acceder al k-esimo elemento es O(1). El orden de este algoritmo siempre es **O(n.log(n))**

### K-Selecciones

#### Orden

Este algoritmo es muy similar al *selection sort*, con la diferencia que sólo se ordenan los k primeros números.

Dada esta condición, se puede ver que el caso óptimo es tener el vector ordenado y k = 0, para el cuál el vector se recorrerá una única vez y se devolverá el primer elemento.

Haciendo la analogía con *selection sort*, el caso pésimo es tener k = n-1, y el vector ordenado de forma invertida para maximizar los swaps.
Al ser k = n-1, es lo mismo que haber hecho un selection sort completo. Se realizan $sum n$ operaciones, es decir $n.(n+1)/2$, por lo que el orden en este caso es n^2


