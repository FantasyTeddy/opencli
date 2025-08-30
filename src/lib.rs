//! This crate provides structures and support for serializing and deserializing [OpenCLI](https://opencli.org/) specifications.
//!
//! # Examples
//!
//! ```no_run
//! use opencli::OpenCliDocument;
//!
//! let opencli = OpenCliDocument::from_path("path/to/opencli.yaml").unwrap();
//! ```

use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

mod error;
pub use error::Error;

/// This is the root object of the OpenCLI Description.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenCliDocument {
    /// The OpenCLI version number
    pub opencli: String,

    /// Information about the CLI
    pub info: OpenCliInfo,

    /// The conventions used by the CLI
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conventions: Option<OpenCliConventions>,

    /// Root command arguments
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub arguments: Vec<OpenCliArgument>,

    /// Root command options
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub options: Vec<OpenCliOption>,

    /// Root command sub commands
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub commands: Vec<OpenCliCommand>,

    /// Root command exit codes
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exit_codes: Vec<OpenCliExitCode>,

    /// Examples of how to use the CLI
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub examples: Vec<String>,

    /// Indicates whether or not the command requires interactive input
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactive: Option<bool>,

    /// Custom metadata
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub metadata: Vec<OpenCliMetadata>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenCliInfo {
    /// The application title
    pub title: String,

    /// A short summary of the application
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    /// A description of the application
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The contact information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<OpenCliContact>,

    /// The application license
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<OpenCliLicense>,

    /// The application version
    pub version: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenCliConventions {
    /// Whether or not grouping of short options are allowed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_options: Option<bool>,

    /// The option argument separator
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_argument_separator: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenCliContact {
    /// The identifying name of the contact person/organization
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The URI for the contact information. This MUST be in the form of a URI
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// The email address of the contact person/organization. This MUST be in the form of an email address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenCliLicense {
    /// The license name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The [SPDX](https://spdx.org/licenses/) license identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenCliCommand {
    /// The command name
    pub name: String,

    /// The command aliases
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub aliases: Vec<String>,

    /// The command's options
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub options: Vec<OpenCliOption>,

    /// The command's arguments
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub arguments: Vec<OpenCliArgument>,

    /// The command's sub commands
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub commands: Vec<OpenCliCommand>,

    /// The command's exit codes
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exit_codes: Vec<OpenCliExitCode>,

    /// The command description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether or not the command is hidden
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,

    /// Examples of how to use the command
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub examples: Vec<String>,

    /// Indicates whether or not the command requires interactive input
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactive: Option<bool>,

    /// Custom metadata
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub metadata: Vec<OpenCliMetadata>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenCliArgument {
    /// The argument name
    pub name: String,

    /// Whether or not the argument is required
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,

    /// The argument arity. Arity defines the minimum and maximum number of argument values
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arity: Option<OpenCliArity>,

    /// A list of accepted values
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub accepted_values: Vec<String>,

    /// The argument group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,

    /// The argument description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether or not the argument is hidden
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,

    /// Custom metadata
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub metadata: Vec<OpenCliMetadata>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenCliOption {
    /// The option name
    pub name: String,

    /// Whether or not the option is required
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,

    /// The option's aliases
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub aliases: Vec<String>,

    /// The option's arguments
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub arguments: Vec<OpenCliArgument>,

    /// The option group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,

    /// The option description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Specifies whether the option is accessible from the immediate parent command and, recursively, from its subcommands
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recursive: Option<bool>,

    /// Whether or not the option is hidden
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,

    /// Custom metadata
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub metadata: Vec<OpenCliMetadata>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenCliArity {
    /// The minimum number of values allowed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i32>,

    /// The maximum number of values allowed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenCliExitCode {
    /// The exit code
    pub code: i32,

    /// The exit code description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenCliMetadata {
    /// The metadata name
    pub name: String,

    /// The metadata value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}

impl OpenCliDocument {
    /// Parse an OpenCLI document from a file.
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let path = path.as_ref();
        let content = fs::read_to_string(path)?;
        Self::from_str(&content)
    }

    /// Parse an OpenCLI document from a byte slice.
    pub fn from_slice(content: &[u8]) -> Result<Self, Error> {
        let content = str::from_utf8(content).map_err(|_| Error::Other("utf8"))?;
        Self::from_str(content)
    }

    /// Parse an OpenCLI document from a string.
    pub fn from_str(content: &str) -> Result<Self, Error> {
        match serde_yaml::from_str(content) {
            Ok(data) => Ok(data),
            Err(_) => {
                let json_data = serde_json::from_str(content)?;
                Ok(json_data)
            }
        }
    }
}
