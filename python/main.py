def residuals(word, code):
    some = {word[len(c):] for c in code if word.startswith(c)}
    return some

word = "hello"
code = {"he", "hell", "lo", "hello"}
print(residuals(word, code))

def residual_sets(code):
    
    # Initialisation avec les résidus des mots du code par rapport à eux-mêmes
    A_current = set()
    for c in code:
        for r in residuals(c, code):
            if r:  # On évite d'ajouter des chaînes vides initialement
                A_current.add(r)
    print(A_current)
    # Itération jusqu'à ce qu'un nouvel ensemble soit vide ou contienne une chaîne vide
    while A_current:
        new_A = set()
        for r in A_current:
            for c in code:
                # Générer de nouveaux mots
                new_word = r + c
                # Trouver les résidus
                new_residuals = residuals(new_word, code)
                if "" in new_residuals:  # Si une chaîne vide est un résidu, le code n'est pas à décodage unique
                    return False
                new_A.update(new_residuals)

        if not new_A:
            break  # Aucun nouveau résidu à ajouter signifie que le code est à décodage unique

        A_current = new_A

    return True

# Exemple d'utilisation
code = {'0', '01', '10'}
result = residual_sets(code)
print("Is the code uniquely decodable?", result)
