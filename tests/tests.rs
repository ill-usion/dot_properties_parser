#[cfg(test)]
mod test {
    use std::path::Path;

    use dot_properties_parser::{
        parse_properties_file, DotPropertiesConfig, PropertyValue,
    };

    #[test]
    fn test_parsing() {
        let result = parse_properties_file(
            Path::new(".common/server.properties"),
            None,
        );
        assert!(result.is_ok())
    }

    #[test]
    fn test_parsing_with_config() {
        let config = DotPropertiesConfig {
            delimiter: ":".to_string(),
            comment_prefix: "!".to_string(),
        };
        let result = parse_properties_file(
            Path::new(".common/custom.server.properties"),
            Some(config),
        );

        assert!(result.is_ok())
    }

    #[test]
    fn test_accessing_props() {
        let props = parse_properties_file(
            Path::new(".common/server.properties"),
            None,
        )
        .unwrap();

        let server_name: &str = props.get("server-name").unwrap().as_ref();
        assert_eq!(server_name, "Dedicated Server")
    }

    #[test]
    fn test_casting() {
        let props = parse_properties_file(
            Path::new(".common/server.properties"),
            None,
        )
        .unwrap();

        let _gamemode: String = props.get("gamemode").unwrap().value_as();
        let _force_gamemode: bool =
            props.get("force-gamemode").unwrap().value_as();
        let _tick_distance: u8 =
            props.get("tick-distance").unwrap().value_as();
        let _server_port: u16 =
            props.get("server-port").unwrap().value_as();
        let _max_players: i32 =
            props.get("max-players").unwrap().value_as();
        let _seed: i64 = props.get("level-seed").unwrap().value_as();
        let _some_float: f32 = props
            .get("player-movement-action-direction-threshold")
            .unwrap()
            .value_as();
    }

    #[test]
    fn test_into() {
        let _integer: PropertyValue = 123.into();
        let _float: PropertyValue = 12.3.into();
        let _bool: PropertyValue = true.into();
        let _size: PropertyValue = 123usize.into();
        let _char: PropertyValue = 'a'.into();

        assert_eq!(_integer.as_ref(), "123");
        assert_eq!(_float.as_ref(), "12.3");
        assert_eq!(_bool.as_ref(), "true");
        assert_eq!(_size.as_ref(), "123");
        assert_eq!(_char.as_ref(), "a");
    }
}
