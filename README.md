# HRDF Parser

HRDF 2.04 (38 fichiers) :
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

HRDF 2.0.5 :
GLEISE_LV95 (fichier pas utilisé dans le code)
GLEISE_WGS (fichier pas utilisé dans le code)

Affichage : ATTRIBUT, BAHNHOF, BETRIEB_DE, BETRIEB_EN, BETRIEB_FR, BETRIEB_IT, DURCHBI, FEIERTAG, GLEIS, GLEIS_LV95, GLEIS_WGS
            INFOTEXT_DE, INFOTEXT_EN, INFOTEXT_FR, INFOTEXT_IT, LINIE, RICHTUNG
Affichage + Algorithme : BFKOORD_LV95, BFKOORD_WGS, ECKDATEN, FPLAN, METABHF
Algorithme : BITFELD, KMINFO, UMSTEIGB, UMSTEIGL, UMSTEIGV, UMSTEIGZ, ZUGART
Incertain : BFPRIOS, BHFART_60
Inutile : ATTRIBUT_DE, ATTRIBUT_EN, ATTRIBUT_FR, ATTRIBUT_IT, BHFART, GREENZHLT, ZEITVS

TODO :
* Refactoring
  * Seperate parser and the rest
  * Errors handling
  * unwrap => except
  * iter vs into_inter ?
  * Delete unnecessary getters
  * Clean the code
* Comments

* Icône vrai point de départ (arrêt)
* Bouton pour annuler

* Async marche pas web service => par_iter

Catégorisation parser :
    auto_increment :
    map + collect :
    vec_to_map before Storage::new :
