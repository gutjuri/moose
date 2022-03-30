use crate::compilation::deprecated_shape::deprecated_shape_support;
use crate::compilation::lowering::lowering;
use crate::compilation::networking::networking_pass;
use crate::compilation::print::print_graph;
use crate::compilation::pruning::prune_graph;
use crate::compilation::toposort::toposort;
use crate::compilation::typing::update_types_one_hop;
use crate::compilation::well_formed::well_formed;
use crate::computation::Computation;
use crate::textual::ToTextual;
use std::convert::TryFrom;

pub mod deprecated_shape;
pub mod lowering;
pub mod networking;
pub mod print;
pub mod pruning;
pub mod toposort;
pub mod typing;
pub mod well_formed;

#[derive(Clone)]
pub enum Pass {
    Networking,
    Print,
    Prune,
    Lowering,
    Toposort,
    Typing,
    WellFormed,
    Dump,
    DeprecatedShape, // Support HostShape in the logical dialect (for pre-0.2.0 computations)
}

impl TryFrom<&str> for Pass {
    type Error = anyhow::Error;

    fn try_from(name: &str) -> anyhow::Result<Pass> {
        match name {
            "networking" => Ok(Pass::Networking),
            "print" => Ok(Pass::Print),
            "prune" => Ok(Pass::Prune),
            "lowering" => Ok(Pass::Lowering),
            "toposort" => Ok(Pass::Toposort),
            "typing" => Ok(Pass::Typing),
            "wellformed" => Ok(Pass::WellFormed),
            "dump" => Ok(Pass::Dump),
            "deprecatedShape" => Ok(Pass::DeprecatedShape),
            missing_pass => Err(anyhow::anyhow!("Unknown pass requested: {}", missing_pass)),
        }
    }
}

impl TryFrom<&String> for Pass {
    type Error = anyhow::Error;
    fn try_from(name: &String) -> anyhow::Result<Pass> {
        Pass::try_from(name.as_str())
    }
}

impl TryFrom<&Pass> for Pass {
    type Error = anyhow::Error;
    fn try_from(pass: &Pass) -> anyhow::Result<Pass> {
        Ok(pass.clone())
    }
}

#[deprecated]
pub const DEFAULT_PASSES: [Pass; 6] = [
    Pass::Typing,
    Pass::DeprecatedShape,
    Pass::Lowering,
    Pass::Prune,
    Pass::Networking,
    Pass::Toposort,
];

#[deprecated]
pub fn compile_passes<'p, P>(
    mut computation: Computation,
    passes: &'p [P],
) -> anyhow::Result<Computation>
where
    Pass: TryFrom<&'p P, Error = anyhow::Error>,
{
    let passes = passes
        .iter()
        .map(Pass::try_from)
        .collect::<anyhow::Result<Vec<Pass>>>()?;

    for pass in passes {
        computation = pass.run(computation)?;
    }
    Ok(computation)
}

pub fn compile<P>(comp: Computation, passes: Option<Vec<P>>) -> anyhow::Result<Computation>
where
    for<'p> Pass: TryFrom<&'p P, Error = anyhow::Error>,
{
    #[allow(deprecated)]
    match passes {
        None => compile_passes::<Pass>(comp, DEFAULT_PASSES.as_slice()),
        Some(passes) => {
            let passes = passes.as_ref();
            compile_passes(comp, passes)
        }
    }
}

impl Pass {
    fn run(&self, comp: Computation) -> anyhow::Result<Computation> {
        match self {
            Pass::Networking => networking_pass(comp),
            Pass::Print => print_graph(comp),
            Pass::Prune => prune_graph(comp),
            Pass::Lowering => lowering(comp),
            Pass::Typing => update_types_one_hop(comp),
            Pass::WellFormed => well_formed(comp),
            Pass::DeprecatedShape => deprecated_shape_support(comp),
            Pass::Dump => {
                println!("{}", comp.to_textual());
                Ok(comp)
            }
            Pass::Toposort => toposort(comp),
        }
    }
}
