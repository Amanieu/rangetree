use rangetree::RangeTree;

#[test]
fn insert_nonoverlap() {
    let mut tree = RangeTree::new();
    tree.insert(0..100, 111);
    assert_eq!(tree.iter().collect::<Vec<_>>(), vec![(0..100, &111),]);
    tree.insert(200..300, 222);
    assert_eq!(
        tree.iter().collect::<Vec<_>>(),
        vec![(0..100, &111), (200..300, &222),]
    );
    tree.insert(500..600, 333);
    assert_eq!(
        tree.iter().collect::<Vec<_>>(),
        vec![(0..100, &111), (200..300, &222), (500..600, &333)]
    );
    tree.insert(400..400, 444);
    assert_eq!(
        tree.iter().collect::<Vec<_>>(),
        vec![(0..100, &111), (200..300, &222), (500..600, &333)]
    );
    tree.insert(400..350, 555);
    assert_eq!(
        tree.iter().collect::<Vec<_>>(),
        vec![(0..100, &111), (200..300, &222), (500..600, &333)]
    );
}

#[test]
fn insert_overlap() {
    let mut tree = RangeTree::new();
    tree.insert(0..100, 111);
    tree.insert(200..300, 222);
    tree.insert(500..600, 333);
    assert_eq!(
        tree.iter().collect::<Vec<_>>(),
        vec![(0..100, &111), (200..300, &222), (500..600, &333)]
    );

    let mut tree2 = tree.clone();
    tree2.insert(50..550, 444);
    assert_eq!(
        tree2.iter().collect::<Vec<_>>(),
        vec![(0..50, &111), (50..550, &444), (550..600, &333)]
    );

    let mut tree2 = tree.clone();
    tree2.insert(100..550, 444);
    assert_eq!(
        tree2.iter().collect::<Vec<_>>(),
        vec![(0..100, &111), (100..550, &444), (550..600, &333)]
    );

    let mut tree2 = tree.clone();
    tree2.insert(150..550, 444);
    assert_eq!(
        tree2.iter().collect::<Vec<_>>(),
        vec![(0..100, &111), (150..550, &444), (550..600, &333)]
    );

    let mut tree2 = tree.clone();
    tree2.insert(200..550, 444);
    assert_eq!(
        tree2.iter().collect::<Vec<_>>(),
        vec![(0..100, &111), (200..550, &444), (550..600, &333)]
    );

    let mut tree2 = tree.clone();
    tree2.insert(220..280, 444);
    assert_eq!(
        tree2.iter().collect::<Vec<_>>(),
        vec![
            (0..100, &111),
            (200..220, &222),
            (220..280, &444),
            (280..300, &222),
            (500..600, &333)
        ]
    );

    let mut tree2 = tree.clone();
    tree2.insert(200..300, 444);
    assert_eq!(
        tree2.iter().collect::<Vec<_>>(),
        vec![(0..100, &111), (200..300, &444), (500..600, &333)]
    );

    let mut tree2 = tree.clone();
    tree2.insert(220..300, 444);
    assert_eq!(
        tree2.iter().collect::<Vec<_>>(),
        vec![
            (0..100, &111),
            (200..220, &222),
            (220..300, &444),
            (500..600, &333)
        ]
    );

    let mut tree2 = tree.clone();
    tree2.insert(200..350, 444);
    assert_eq!(
        tree2.iter().collect::<Vec<_>>(),
        vec![(0..100, &111), (200..350, &444), (500..600, &333)]
    );

    let mut tree2 = tree.clone();
    tree2.insert(220..350, 444);
    assert_eq!(
        tree2.iter().collect::<Vec<_>>(),
        vec![
            (0..100, &111),
            (200..220, &222),
            (220..350, &444),
            (500..600, &333)
        ]
    );
}

#[test]
fn remove() {
    let mut tree = RangeTree::new();
    tree.insert(0..100, 111);
    tree.insert(200..300, 222);
    tree.insert(500..600, 333);
    assert_eq!(
        tree.iter().collect::<Vec<_>>(),
        vec![(0..100, &111), (200..300, &222), (500..600, &333)]
    );

    let mut tree2 = tree.clone();
    tree2.remove(0..0);
    assert_eq!(
        tree2.iter().collect::<Vec<_>>(),
        vec![(0..100, &111), (200..300, &222), (500..600, &333)]
    );

    let mut tree2 = tree.clone();
    tree2.remove(100..0);
    assert_eq!(
        tree2.iter().collect::<Vec<_>>(),
        vec![(0..100, &111), (200..300, &222), (500..600, &333)]
    );

    let mut tree2 = tree.clone();
    tree2.remove(0..50);
    assert_eq!(
        tree2.iter().collect::<Vec<_>>(),
        vec![(50..100, &111), (200..300, &222), (500..600, &333)]
    );

    let mut tree2 = tree.clone();
    tree2.remove(50..550);
    assert_eq!(
        tree2.iter().collect::<Vec<_>>(),
        vec![(0..50, &111), (550..600, &333)]
    );

    let mut tree2 = tree.clone();
    tree2.remove(50..100);
    assert_eq!(
        tree2.iter().collect::<Vec<_>>(),
        vec![(0..50, &111), (200..300, &222), (500..600, &333)]
    );

    let mut tree2 = tree.clone();
    tree2.remove(220..280);
    assert_eq!(
        tree2.iter().collect::<Vec<_>>(),
        vec![
            (0..100, &111),
            (200..220, &222),
            (280..300, &222),
            (500..600, &333)
        ]
    );
}
