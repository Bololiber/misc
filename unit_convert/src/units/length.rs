use crate::impl_units;

impl_units! {
    Length => {
        Meter => [
            <| |m| *m,
            |> |m| *m,
            aliases = ["m", "metre"],
            metric = true
        ],
        Inch => [
            <| |i| i * 0.0254,
            |> |m| m / 0.0254,
            aliases = ["in"]
        ],
        Thou => [
            <| |th| th * 0.0000254,
            |> |m| m / 0.0000254,
            aliases = ["mil"]
        ],
        Foot => [
            <| |f| f * 0.3048,
            |> |m| m / 0.3048,
            aliases = ["ft", "foot"]
        ],
        Yard => [
            <| |y| y * 0.9144,
            |> |m| m / 0.9144,
            aliases = ["yd"]
        ],
        Mile => [
            <| |mi| mi * 1609.344,
            |> |m| m / 1609.344,
            aliases = ["mi"]
        ],
        League => [
            <| |l| l * 4828.0417,
            |> |m| m / 4828.0417
        ],
        Light-Year => [
            <| |ly| ly * 9460730472580800,
            |> |m| m / 9460730472580800,
            aliases = ["ly"]
        ],
        Astronomical Unit => [
            <| |au| au * 1.495978707e11,
            |> |m| m / 1.495978707e11,
            aliases = ["au"]
        ],
        Siriometer => [
            <| |sir| sir * 1.495978707e17,
            |> |m| m / 1.495978707e17,
            aliases = ["sir"]
        ],
        Parsec => [
            <| |pc| pc * 3.0856776e16,
            |> |m| m / 3.0856776e16,
            aliases = ["pc"]
        ],
        Potrzebie => [
            <| |p| p * 0.0022633484517438173216473,
            |> |m| m / 0.0022633484517438173216473,
            aliases = ["p"]
        ],
        Farshimmelt Potrzebie => [
            <| |fp| fp * 0.0022633484517438173216473e-6,
            |> |m| m / 0.0022633484517438173216473e-6,
            aliases = ["fp"]
        ],
        Furshlugginer Potrezebie => [
            <| |fup| fup * 0.0022633484517438173216473e6,
            |> |m| m / 0.0022633484517438173216473e6,
            aliases = ["fup"]
        ],
        Furlong => [
            <| |fur| fur * 201.168,
            |> |m| m / 201.168,
            aliases = ["fur"]
        ]
    }
}
