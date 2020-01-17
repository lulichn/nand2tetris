package net.lulichn.n2t.compiler;

import org.antlr.v4.runtime.tree.ParseTree;
import org.antlr.v4.runtime.tree.TerminalNodeImpl;
import org.w3c.dom.Document;
import org.w3c.dom.Node;

import javax.xml.parsers.DocumentBuilderFactory;
import javax.xml.parsers.ParserConfigurationException;
import javax.xml.transform.OutputKeys;
import javax.xml.transform.Transformer;
import javax.xml.transform.TransformerException;
import javax.xml.transform.TransformerFactory;
import javax.xml.transform.dom.DOMSource;
import javax.xml.transform.stream.StreamResult;
import java.io.OutputStream;
import java.io.StringWriter;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

public class ToXml {
  static final List<String> keywords = Arrays.asList("class", "constructor", "function", "method", "field", "static", "var", "int", "char", "boolean", "void", "true", "false", "null", "this", "let", "do", "if", "else",
      "while", "return");

  private final Document doc;

  public ToXml(Document doc) {
    this.doc = doc;
  }

  public static Node klass(ParseTree klass) throws ParserConfigurationException {
    var doc = createXMLDocument("root");
    var toXml = new ToXml(doc);
    var xml = new ToXml(doc).make(klass);
    return xml.get(0);
  }

  public static Document createXMLDocument(String root) throws ParserConfigurationException {
    DocumentBuilderFactory factory = DocumentBuilderFactory.newInstance();
    var builder = factory.newDocumentBuilder();

    var dom = builder.getDOMImplementation();
    return dom.createDocument("", root, null);
  }

  public List<Node> make(ParseTree tree) {
    if (tree instanceof JackParser.IdentifierContext) {
      return List.of(text("identifier", tree.getText()));
    }

    if (tree instanceof JackParser.StringConstantContext) {
      return List.of(text("stringConstant", tree.getText().substring(1, tree.getText().length() - 1)));
    }

    if (tree instanceof JackParser.IntegerConstantContext) {
      return List.of(text("integerConstant", tree.getText()));
    }

    if (tree instanceof TerminalNodeImpl) {
      var text = tree.getText();
      if (keywords.contains(text)) {
        return List.of(text("keyword", text));
      } else {
        return List.of(text("symbol", text));
      }
    }

    var list = new ArrayList<Node>();
    for (int id = 0; id < tree.getChildCount(); id++) {
      list.addAll(make(tree.getChild(id)));
    }

    if (tree instanceof JackParser.ClassNameContext ||
        tree instanceof JackParser.SubroutineKindContext ||
        tree instanceof JackParser.ReturnTypeContext ||
        tree instanceof JackParser.SubroutineNameContext ||
        tree instanceof JackParser.TypeContext ||
        tree instanceof JackParser.VarNameContext ||
        tree instanceof JackParser.QualifierContext ||
        tree instanceof JackParser.StatementContext ||
        tree instanceof JackParser.OpContext ||
        tree instanceof JackParser.UnaryOpContext ||
        tree instanceof JackParser.KeywordConstantContext ||
        // tests
        tree instanceof JackParser.ElseClauseContext ||
        tree instanceof JackParser.ArrayIndexingContext ||
        tree instanceof JackParser.VarTypeContext ||
        tree instanceof JackParser.VarListContext ||
        tree instanceof JackParser.TypedVarContext ||
        tree instanceof JackParser.SubroutineCallContext) {
      return list;
    }

    String containerName = "";
    if (tree instanceof JackParser.VarDecContext) {
      containerName = "varDec";
    } else if (tree instanceof JackParser.ClassVarDecContext) {
      containerName = "classVarDec";
    } else if (tree instanceof JackParser.LetStatementContext) {
      containerName = "letStatement";
    } else if (tree instanceof JackParser.IfStatementContext) {
      containerName = "ifStatement";
    } else if (tree instanceof JackParser.WhileStatementContext) {
      containerName = "whileStatement";
    } else if (tree instanceof JackParser.DoStatementContext) {
      containerName = "doStatement";
    } else if (tree instanceof JackParser.ReturnStatementContext) {
      containerName = "returnStatement";
    } else if (tree instanceof JackParser.TermContext) {
      containerName = "term";
    } else if (tree instanceof JackParser.ExpressionContext) {
      containerName = "expression";
    } else if (tree instanceof JackParser.StatementsContext) {
      containerName = "statements";
    } else if (tree instanceof JackParser.SubroutineDecContext) {
      containerName = "subroutineDec";
    } else if (tree instanceof JackParser.SubroutineBodyContext) {
      containerName = "subroutineBody";
    } else if (tree instanceof JackParser.KlassContext) {
      containerName = "class";
    }
    //
//      else if (tree instanceof JackParser.VarListContext) {
//        containerName = "varList";
    else if (tree instanceof JackParser.ParameterListContext) {
      containerName = "parameterList";
    } else if (tree instanceof JackParser.ExpressionListContext) {
      containerName = "expressionList";
    }

    var parent = doc.createElement(containerName);
    for (var c: list) {
      parent.appendChild(c);
    }

    return List.of(parent);
  }

  Node text(String name, String value) {
    var node = doc.createElement(name);
    var textNode = doc.createTextNode(" " + value + " ");
    node.appendChild(textNode);
    return node;
  }
}