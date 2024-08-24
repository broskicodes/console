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
    "nodeLabels": [
      {
        "$id": "user",
        "token": "User"
      },
      {
        "$id": "interest",
        "token": "Interest"
      },
      {
        "$id": "goal",
        "token": "Goal"
      },
      {
        "$id": "motivation",
        "token": "Motivation"
      },
      {
        "$id": "task",
        "token": "Task"
      },
      {
        "$id": "date",
        "token": "Date"
      }
    ],
    "relationshipTypes": [
      {
        "$id": "interestedIn",
        "token": "INTERESTED_IN"
      },
      {
        "$id": "linkedTo",
        "token": "LINKED_TO"
      },
      {
        "$id": "hasGoal",
        "token": "HAS_GOAL"
      },
      {
        "$id": "motivatedBy",
        "token": "MOTIVATED_BY"
      },
      {
        "$id": "partOf",
        "token": "PART_OF"
      },
      {
        "$id": "createdOn",
        "token": "CREATED_ON"
      }
    ],
    "nodeObjectTypes": [
      {
        "$id": "user",
        "labels": [
          {
            "$ref": "user"
          }
        ],
        "properties": [
          {
            "token": "user_id",
            "nullable": false,
            "type": "string"
          }
        ]
      },
      {
        "$id": "interest",
        "labels": [
          {
            "$ref": "interest"
          }
        ],
        "properties": [
          {
            "token": "name",
            "nullable": false,
            "type": "string"
          }
        ]
      },
      {
        "$id": "goal",
        "labels": [
          {
            "$ref": "goal"
          }
        ],
        "properties": [
          {
            "token": "description",
            "nullable": false,
            "type": "string"
          },
          {
            "token": "timeframe",
            "nullable": true,
            "type": "string",
            "enum": [
              "short-term",
              "medium-term",
              "long-term"
            ]
          }
        ]
      },
      {
        "$id": "motivation",
        "labels": [
          {
            "$ref": "motivation"
          }
        ],
        "properties": [
          {
            "token": "title",
            "nullable": false,
            "type": "string"
          },
          {
            "token": "reason",
            "nullable": false,
            "type": "string"
          }
        ]
      },
      {
        "$id": "task",
        "labels": [
          {
            "$ref": "task"
          }
        ],
        "properties": [
          {
            "token": "action",
            "nullable": false,
            "type": "string"
          },
          {
            "token": "status",
            "nullable": true,
            "type": "string",
            "enum": [
              "pending",
              "in_progress",
              "completed",
              "failed"
            ]
          }
        ]
      },
      {
        "$id": "date",
        "labels": [
          {
            "$ref": "date"
          }
        ],
        "properties": [
          {
            "token": "day",
            "nullable": false,
            "type": "integer"
          },
          {
            "token": "month",
            "nullable": false,
            "type": "integer"
          },
          {
            "token": "year",
            "nullable": false,
            "type": "integer"
          }
        ]
      }
    ],
    "relationshipObjectTypes": [
      {
        "$id": "userHasInterest",
        "type": {
          "$ref": "interestedIn"
        },
        "from": {
          "$ref": "user"
        },
        "to": {
          "$ref": "interest"
        },
        "properties": []
      },
      {
        "$id": "userHasGoal",
        "type": {
          "$ref": "hasGoal"
        },
        "from": {
          "$ref": "user"
        },
        "to": {
          "$ref": "goal"
        },
        "properties": []
      },
      {
        "$id": "goalLinkedToInterest",
        "type": {
          "$ref": "linkedTo"
        },
        "from": {
          "$ref": "goal"
        },
        "to": {
          "$ref": "interest"
        },
        "properties": []
      },
      {
        "$id": "goalIsMotivatedBy",
        "type": {
          "$ref": "motivatedBy"
        },
        "from": {
          "$ref": "goal"
        },
        "to": {
          "$ref": "motivation"
        },
        "properties": []
      },
      {
        "$id": "taskBelongsToGoal",
        "type": {
          "$ref": "partOf"
        },
        "from": {
          "$ref": "task"
        },
        "to": {
          "$ref": "goal"
        },
        "properties": []
      },
      {
        "$id": "taskCreatedOn",
        "type": {
          "$ref": "createdOn"
        },
        "from": {
          "$ref": "task"
        },
        "to": {
          "$ref": "date"
        },
        "properties": []
      }
    ]
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
    