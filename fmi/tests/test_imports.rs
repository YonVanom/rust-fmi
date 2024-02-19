use fmi::{FmiImport as _, FmiInstance as _};

const FMU2_NAMES: [&str; 6] = [
    "BouncingBall",
    "Dahlquist",
    "Feedthrough",
    "Resource",
    "Stair",
    "VanDerPol",
];

const FMU3_NAMES: [&str; 8] = [
    "BouncingBall",
    "Clocks",
    "Dahlquist",
    "Feedthrough",
    "Resource",
    "Stair",
    "StateSpace",
    "VanDerPol",
];

#[test]
fn test_fmi2_imports() {
    let mut ref_fmus = test_data::ReferenceFmus::new().unwrap();

    for &name in FMU2_NAMES.iter() {
        let import = ref_fmus
            .get_reference_fmu(name, "2.0")
            .unwrap()
            .as_fmi2()
            .expect("Expected FMI2 import");
        assert_eq!(import.model_description().fmi_version, "2.0");

        #[cfg(target_os = "linux")]
        {
            if import.model_description().model_exchange.is_some() {
                let me = import.instantiate_me("inst1", false, true).unwrap();
                assert_eq!(me.get_version(), "2.0");
            }

            if import.model_description().co_simulation.is_some() {
                let cs = import.instantiate_cs("inst1", false, true).unwrap();
                assert_eq!(cs.get_version(), "2.0");
            }
        }
    }
}

#[test_log::test]
fn test_fmi3_imports() {
    let mut ref_fmus = test_data::ReferenceFmus::new().unwrap();

    for &name in FMU3_NAMES.iter() {
        let import = ref_fmus
            .get_reference_fmu(name, "3.0")
            .unwrap()
            .as_fmi3()
            .expect("Expected FMI3 import");
        assert_eq!(import.model_description().fmi_version, "3.0");

        if import.model_description().model_exchange.is_some() {
            let me = import.instantiate_me("inst1", false, true).unwrap();
            assert_eq!(me.get_version(), "3.0");
        }

        if import.model_description().co_simulation.is_some() {
            let cs = import
                .instantiate_cs("inst1", false, true, false, false, &[])
                .unwrap();
            assert_eq!(cs.get_version(), "3.0");
        }
    }
}
