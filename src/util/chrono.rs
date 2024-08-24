use std::thread;
use std::time::{Duration, Instant};

// removethe "_" when we will use this function in future.
pub fn _run_with_timeout<F>(timeout: Duration, process: F)
where
    F: FnOnce() + Send + 'static,
{
    // Démarrer le chronomètre
    let start_time = Instant::now();

    // Créer un thread pour exécuter le processus
    let handle = thread::spawn(process);

    // Boucle pour vérifier le temps écoulé
    loop {
        // Vérifier si la durée limite est écoulée
        if start_time.elapsed() > timeout {
            println!("Temps écoulé. Arrêt du processus.");
            // Tuer le processus en quittant la fonction (le thread s'arrête lorsque le programme se termine)
            return;
        }

        // Vérifier si le processus est terminé
        if handle.is_finished() {
            println!("Processus terminé avant le temps imparti.");
            break;
        }

        // Dormir un peu pour éviter de boucler trop rapidement
        thread::sleep(Duration::from_millis(100));
    }
}
