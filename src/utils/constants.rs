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
  "graphSchemaRepresentation": {
    "version": "1.0",
    "graphSchema": {
      "nodeLabels": [
        {
          "$id": "User",
          "token": "User"
        },
        {
          "$id": "Interest",
          "token": "Interest"
        },
        {
          "$id": "Goal",
          "token": "Goal"
        },
        {
          "$id": "Motivation",
          "token": "Motivation"
        },
        {
          "$id": "Task",
          "token": "Task"
        }
      ],
      "relationshipTypes": [
        {
          "$id": "hasInterest",
          "token": "HAS_INTEREST"
        },
        {
          "$id": "hasGoal",
          "token": "HAS_GOAL"
        },
        {
          "$id": "isMotivatedBy",
          "token": "IS_MOTIVATED_BY"
        },
        {
          "$id": "hasTask",
          "token": "HAS_TASK"
        }
      ],
      "nodeObjectTypes": [
        {
          "$id": "User",
          "labels": [
            {
              "$ref": "User"
            }
          ],
          "properties": [
            {
              "token": "name",
              "nullable": false,
              "type": "string"
            },
            {
              "token": "profession",
              "nullable": true,
              "type": "string"
            }
          ]
        },
        {
          "$id": "Interest",
          "labels": [
            {
              "$ref": "Interest"
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
          "$id": "Goal",
          "labels": [
            {
              "$ref": "Goal"
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
              "type": "string"
            }
          ]
        },
        {
          "$id": "Motivation",
          "labels": [
            {
              "$ref": "Motivation"
            }
          ],
          "properties": [
            {
              "token": "reason",
              "nullable": false,
              "type": "string"
            }
          ]
        },
        {
          "$id": "Task",
          "labels": [
            {
              "$ref": "Task"
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
              "type": "string"
            }
          ]
        }
      ],
      "relationshipObjectTypes": [
        {
          "$id": "UserHasInterest",
          "type": {
            "$ref": "hasInterest"
          },
          "from": {
            "$ref": "User"
          },
          "to": {
            "$ref": "Interest"
          },
          "properties": []
        },
        {
          "$id": "UserHasGoal",
          "type": {
            "$ref": "hasGoal"
          },
          "from": {
            "$ref": "User"
          },
          "to": {
            "$ref": "Goal"
          },
          "properties": []
        },
        {
          "$id": "GoalIsMotivatedBy",
          "type": {
            "$ref": "isMotivatedBy"
          },
          "from": {
            "$ref": "Goal"
          },
          "to": {
            "$ref": "Motivation"
          },
          "properties": []
        },
        {
          "$id": "TaskBelongsToGoal",
          "type": {
            "$ref": "hasTask"
          },
          "from": {
            "$ref": "Goal"
          },
          "to": {
            "$ref": "Task"
          },
          "properties": []
        }
      ]
    }
  }
}"##;