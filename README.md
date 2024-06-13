Progression (38/38) :
* ATTRIBUT
* ATTRIBUT_DE (fichier pas utilisé dans le code)
* ATTRIBUT_EN (fichier pas utilisé dans le code)
* ATTRIBUT_FR (fichier pas utilisé dans le code)
* ATTRIBUT_IT (fichier pas utilisé dans le code)
* BAHNHOF
* BETRIEB_DE
* BETRIEB_EN
* BETRIEB_FR
* BETRIEB_IT
* BFKOORD_LV95
* BFKOORD_WGS
* BFPRIOS
* BHFART (fichier pas utilisé dans le code)
* BHFART_60
* BITFELD
* DURCHBI
* ECKDATEN
* FEIERTAG
* FPLAN
* GLEIS
* GLEIS_LV95
* GLEIS_WGS
* GRENZHLT (fichier pas utilisé dans le code)
* INFOTEXT_DE
* INFOTEXT_EN
* INFOTEXT_FR
* INFOTEXT_IT
* KMINFO
* LINIE
* METABHF
* RICHTUNG
* UMSTEIGB
* UMSTEIGL
* UMSTEIGV
* UMSTEIGZ
* ZUGART
* ZEITVS (fichier pas utilisé dans le code)

Affichage : ATTRIBUT, BAHNHOF, BETRIEB_DE, BETRIEB_EN, BETRIEB_FR, BETRIEB_IT, BFKOORD_LV95, BFKOORD_WGS, DURCHBI, FEIERTAG, GLEIS, GLEIS_LV95, GLEIS_WGS
            INFOTEXT_DE, INFOTEXT_EN, INFOTEXT_FR, INFOTEXT_IT, LINIE, RICHTUNG, ZUGART
Affichage + Algorithme : ECKDATEN, FPLAN, METABHF
Algorithme : BFPRIOS, BITFELD, KMINFO, UMSTEIGB, UMSTEIGL, UMSTEIGV, UMSTEIGZ
Incertain : BHFART_60
Inutile : ATTRIBUT_DE, ATTRIBUT_EN, ATTRIBUT_FR, ATTRIBUT_IT, BHFART, GREENZHLT, ZEITVS

TODO :
* Ajouter thiserror

Algorithme de calcul du trajet le plus court (Résumé) :
* Maximum N connexions
* Seulement les trajets possibles
* Ne pas considérer les arrêts où les changements sont désactivés pour les connexions
* Blocage des boucles
  * On ne revient pas sur ses pas
  * Arrêter de suivre un trajet dès qu'il boucle sur lui-même
* Si une solution est trouvée, alors il faut arrêter les routes qui arriveraient de toute manière plus tard que celle-ci
* Empêcher de réemprunter le même type de trajet que précédemment (ex. sortir du 14 pour reprendre le 14 d'après)
* Filter les connexions et ne prendre que des trajets avec une route unique
  * Par exemple si la ligne 21 passe 10 fois, seulement les 2 trajets (1 dans chaque sens) arrivant le plus tôt sont considérés
* Considérer pour les connexions, tous les trajets passant jusqu'à une certaine heure (ex. heure de départ + 4 heures)
* Considérer le changement d'arrêt à pied (ex. "Genève, gare" vers "Genève")
* Pouvoir calculer un trajet sur 2 jours
* La route qui arrive en premier (le plus tôt) à un arrêt est la seule qui peut explorer les connexions
    * Les routes qui arrivent plus tard peuvent seulement suivre leur trajet jusqu'à la fin
* Si une journey a déjà été emprunté avec moins de connexions alors il n'est pas possible de l'emprunter lors de l'exploration des connexions
    * L'emprenter avec plus de connexions ne peut pas améliorer la solution

Algorithme de calcul du trajet le plus court (TODO) :
* Considérer les temps de transferts lors d'une correspondance
* Depuis un arrêt d'origine, pouvoir calculer toutes les routes vers tous les arrêts atteignables dans un temps de donner (ex. en 2 heures)

Algorithme de calcul du trajet le plus court (Problèmes) :
* Lent quand la solution requiert beaucoup de connexions
* Lent quand le temps d'arrivée de la solution est tard  (à partir de 3-4 heures plus tard que l'heure de départ)
* Ne maximise pas le temps de départ

Algorithme de calcul du trajet le plus court (Optionnel) :
* Pouvoir paginer les résultats
    * Récupérer N résults plus tôt
    * Récupérer N résults plus tard
* Pouvoir préciser une heure de départ ou d'arrivée
* Renvoyer les résultats via une structure

Forcer l'utilisation de train quand l'arrêt d'arrivée est loin
