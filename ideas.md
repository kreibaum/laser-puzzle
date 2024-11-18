# Lageregeln

- Ganz am Rand rein leuchten und an der Seite raus.

        × ⇄ × × C × × E
      × ? ? ? ? ? ? . . E
      ⇄ ? ? ? ? ? ? o . ×
      D ? ? ? ? ? ? ? ? ⇄

- Hinter einer Reflektion ist immer min. ein Feld frei.

- **IMPLEMENTED** Wenn ein Buchstabe am Rand ist, dann sind vier Felder frei: (Beispiel B)

      × ? ? ? . ? ? ? . ⇄
      A ? ? . . . ? ? ? B 
      . × A × B C D × ×

- Ein × am Rand mit einem freien Feld direkt daneben bedeutet,
  dass die diagonal-Felder auch frei sind.

~ Ein × am Rand mit freier Fläche "davor und bis zum Rand darunter" gibt uns auch eine freie Reihe.

~ Ein ⇄ am Rand mit freier Fläche "davor und bis zum Rand darunter" sowie einem Atom in der Reihe
muss von einem Atom was direkt am Rand diagonal zum ⇄ liegt reflektiert werden.

- In den drei vollständigen Reihen links, auf, rechts von einem × muss mindestens ein
  Feld ein Atom haben.

# Strahlen verfolgen

- Wenn eine nicht-absorbtion am Rand ist, dann geht er nicht gerade auf ein Atom zu, ohne abgelenkt zu werden.

# Brute Force

Wenn nur noch ein Atom fehlt, dann mach Brute Force.

# Informationen enfternen

- Einige Informationen können entfernt werden.