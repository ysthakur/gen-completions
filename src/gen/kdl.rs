use kdl::{KdlDocument, KdlEntry, KdlNode};

use crate::gen::CommandInfo;

/// Turn a [`CommandInfo`] into a [`KdlNode`]
pub fn to_kdl_node(cmd: &CommandInfo) -> KdlNode {
  let mut node = KdlNode::new(cmd.name.to_string());
  let mut children = KdlDocument::new();

  let mut flag_nodes = KdlDocument::new();

  for flag in &cmd.flags {
    let mut flag_node = KdlNode::new("-");

    for form in &flag.forms {
      flag_node
        .entries_mut()
        .push(KdlEntry::new(form.to_string()));
    }

    if let Some(desc) = &flag.desc {
      let mut description_node = KdlNode::new("desc");
      description_node
        .entries_mut()
        .push(KdlEntry::new(desc.to_string()));

      let mut flag_children = KdlDocument::new();
      flag_children.nodes_mut().push(description_node);
      flag_node.set_children(flag_children);
    }

    flag_nodes.nodes_mut().push(flag_node);
  }

  let mut flags = KdlNode::new("flags");
  flags.set_children(flag_nodes);
  children.nodes_mut().push(flags);

  let mut subcommands = KdlDocument::new();
  for subcmd in &cmd.subcommands {
    subcommands.nodes_mut().push(to_kdl_node(subcmd));
  }

  let mut subcommands_node = KdlNode::new("subcommands");
  subcommands_node.set_children(subcommands);
  children.nodes_mut().push(subcommands_node);

  node.set_children(children);
  node
}
