use atomas_core::{
    elements::{Element, ElementType, SpecialAtom},
    ring::{CircularList, AdjMatrix},
};

#[test]
fn test_adjmatrix() {
    let element = Element {
        element_type: ElementType::Periodic(1),
        symbol: "H",
        name: "Hydrogen",
        color: (255, 255, 255),
    };
    let special = Element {
        element_type: ElementType::Special(SpecialAtom::Plus),
        symbol: "+",
        name: "Plus",
        color: (255, 255, 255),
    };
    let custom = Element {
        element_type: ElementType::Custom(119),
        symbol: "C",
        name: "Carbon",
        color: (255, 255, 255),
    };

    let mut ring = CircularList::new();
    ring.push(element);
    ring.push(special);
    ring.push(custom);

    let adjmatrix = AdjMatrix::new(&ring);
    println!("{}", adjmatrix);
}

