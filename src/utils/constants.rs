pub const NEO4J_SCHEMA_DEFINITION: &str = r##"{
  "type": "object",
  "properties": {
    "$schema": {
      "type": "string"
    },
    "graphSchemaRepresentation": {
      "type": "object",
      "properties": {
        "version": {
          "type": "string"
        },
        "graphSchema": {
          "type": "object",
          "properties": {
            "nodeLabels": {
              "type": "array",
              "items": {
                "type": "object",
                "properties": {
                  "$id": {
                    "type": "string"
                  },
                  "token": {
                    "type": "string"
                  }
                },
                "additionalProperties": false,
                "required": ["$id", "token"]
              }
            },
            "relationshipTypes": {
              "type": "array",
              "items": {
                "type": "object",
                "properties": {
                  "$id": {
                    "type": "string"
                  },
                  "token": {
                    "type": "string"
                  }
                },
                "additionalProperties": false,
                "required": ["$id", "token"]
              }
            },
            "nodeObjectTypes": {
              "type": "array",
              "items": {
                "type": "object",
                "properties": {
                  "$id": {
                    "type": "string"
                  },
                  "labels": {
                    "type": "array",
                    "items": {
                      "type": "object",
                      "properties": {
                        "$ref": {
                          "type": "string"
                        }
                      },
                      "additionalProperties": false,
                      "required": ["$ref"]
                    }
                  },
                  "properties": { "$ref": "#/$defs/PropertyTypesOneOf" }
                },
                "additionalProperties": false,
                "required": ["$id", "labels", "properties"]
              }
            },
            "relationshipObjectTypes": {
              "type": "array",
              "items": {
                "type": "object",

                "properties": {
                  "$id": {
                    "type": "string"
                  },
                  "type": {
                    "type": "object",
                    "additionalProperties": false,
                    "required": ["$ref"],
                    "properties": {
                      "$ref": {
                        "type": "string"
                      }
                    }
                  },
                  "from": {
                    "type": "object",
                    "additionalProperties": false,
                    "required": ["$ref"],
                    "properties": {
                      "$ref": {
                        "type": "string"
                      }
                    }
                  },
                  "to": {
                    "type": "object",
                    "additionalProperties": false,
                    "required": ["$ref"],
                    "properties": {
                      "$ref": {
                        "type": "string"
                      }
                    }
                  },
                  "properties": { "$ref": "#/$defs/PropertyTypesOneOf" }
                },
                "additionalProperties": false,
                "required": ["$id", "type", "from", "to", "properties"]
              }
            }
          },
          "additionalProperties": false,
          "required": [
            "nodeLabels",
            "relationshipTypes",
            "nodeObjectTypes",
            "relationshipObjectTypes"
          ]
        }
      }
    }
  },
  "additionalProperties": false,
  "required": ["graphSchemaRepresentation"],
  "$defs": {
    "PropertyTypesOneOf": {
      "type": "array",
      "items": {
        "oneOf": [
          {
            "$ref": "#/$defs/PropertyTypesArrayObject"
          },
          {
            "type": "object",
            "additionalProperties": false,
            "required": ["token", "type", "nullable"],
            "properties": {
              "token": {
                "type": "string"
              },
              "nullable": { "type": "boolean" },
              "$id": { "type": "string" },
              "type": {
                "oneOf": [
                  {
                    "$ref": "#/$defs/PropertyTypes"
                  },
                  {
                    "type": "array",
                    "items": {
                      "oneOf": [
                        { "$ref": "#/$defs/PropertyTypes" },
                        { "$ref": "#/$defs/PropertyTypesArray" }
                      ]
                    }
                  }
                ]
              }
            }
          }
        ]
      }
    },
    "PropertyTypesArray": {
      "type": "object",
      "required": ["items", "type"],
      "properties": {
        "type": {
          "type": "string",
          "pattern": "^array$"
        },
        "items": {
          "$ref": "#/$defs/PropertyTypes"
        }
      }
    },
    "PropertyTypesArrayObject": {
      "type": "object",
      "additionalProperties": false,
      "required": ["token", "type", "nullable"],
      "properties": {
        "token": {
          "type": "string"
        },
        "type": {
          "$ref": "#/$defs/PropertyTypesArray"
        },
        "nullable": { "type": "boolean" }
      }
    },
    "PropertyTypes": {
      "type": "object",
      "properties": {
        "type": {
          "$ref": "#/$defs/PropertyTypesEnum"
        }
      }
    },
    "PropertyTypesEnum": {
      "type": "string",
      "enum": [
        "integer",
        "string",
        "float",
        "boolean",
        "point",
        "date",
        "datetime",
        "time",
        "localtime",
        "localdatetime",
        "duration"
      ]
    }
  }
}"##;

pub const GRAPH_SCHEMA: &str = r##"{
  "graphSchema": {
    "nodeTypes": [
      {
        "id_format": "user_{num}",
        "label": "User",
        "properties": {
          "user_id": {
            "nullable": false,
            "type": "string"
          }
        }
      },
      {
        "id_format": "interest_{num}",
        "label": "Interest",
        "properties": {
          "name": {
            "nullable": false,
            "type": "string"
          }
        }
      },
      {
        "id_format": "goal_{num}",
        "label": "Goal",
        "properties": {
          "description": {
            "nullable": false,
            "type": "string"
          },
          "timeframe": {
            "nullable": true,
            "type": "string",
            "enum": [
              "short-term",
              "medium-term",
              "long-term"
            ]
          }
        }
      },
      {
        "id_format": "motivation_{num}",
        "label": "Motivation",
        "properties": {
          "title": {
            "nullable": false,
            "type": "string"
          },
          "reason": {
            "nullable": false,
            "type": "string"
          }
        }
      },
      {
        "id_format": "task_{num}",
        "label": "Task",
        "properties": {
          "action": {
            "nullable": false,
            "type": "string"
          },
          "status": {
            "nullable": true,
            "type": "string",
            "enum": [
              "pending",
              "in_progress",
              "completed",
              "failed"
            ]
          }
        }
      },
      {
        "id_format": "date_{num}",
        "label": "Date",
        "properties": {
          "day": {
            "nullable": false,
            "type": "integer"
          },
          "month": {
            "nullable": false,
            "type": "integer"
          },
          "year": {
            "nullable": false,
            "type": "integer"
          }
        }
      }
    ],
    "relationshipTypes": [
      {
        "label": "INTERESTED_IN",
        "source_node_type": "User",
        "target_node_type": "Interest"
      },
      {
        "label": "LINKED_TO",
        "source_node_type": "Goal",
        "target_node_type": "Interest"
      },
      {
        "label": "HAS_GOAL",
        "source_node_type": "User",
        "target_node_type": "Goal"
      },
      {
        "label": "MOTIVATED_BY",
        "source_node_type": "Goal",
        "target_node_type": "Motivation"
      },
      {
        "label": "PART_OF",
        "source_node_type": "Task",
        "target_node_type": "Goal"
      },
      {
        "label": "CREATED_ON",
        "source_node_type": "Task",
        "target_node_type": "Date"
      }
    ],      
  }
}"##;

pub const GRAPH_DATA_DEF: &str = r#"{
    "type": "object",
    "properties": {
        "nodes": {
            "type": "array",
            "items": {
            "type": "object",
            "properties": {
                "id": {
                "type": "string"
                },
                "label": {
                "type": "string"
                },
                "props": {
                "type": "object",
                "additionalProperties": true
                }
            },
            "required": ["id", "label", "props"]
            }
        },
        "relationships": {
            "type": "array",
            "items": {
            "type": "object",
            "properties": {
                "source": {
                "type": "string"
                },
                "target": {
                "type": "string"
                },
                "label": {
                "type": "string"
                }
            },
            "required": ["source", "target", "label"]
            }
        }
    },
    "required": ["nodes", "relationships"]
}"#;
