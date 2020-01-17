package net.lulichn.n2t.compiler;

import org.antlr.v4.runtime.CharStreams;
import org.antlr.v4.runtime.CommonTokenStream;
import org.w3c.dom.Node;

import javax.xml.parsers.ParserConfigurationException;
import javax.xml.transform.OutputKeys;
import javax.xml.transform.Transformer;
import javax.xml.transform.TransformerException;
import javax.xml.transform.TransformerFactory;
import javax.xml.transform.dom.DOMSource;
import javax.xml.transform.stream.StreamResult;
import java.io.*;
import java.nio.file.Paths;

public class Main {
  public static void main(String[] args) throws IOException, ParserConfigurationException, TransformerException {
    var path = Paths.get(args[0]);

    var lexer = new JackLexer(CharStreams.fromPath(path));
    var stream = new CommonTokenStream(lexer);
    var parser = new JackParser(stream);

    var node = ToXml.klass(parser.klass());

    var name = getNameWithoutExtension(args[0]);
    var os = new FileOutputStream(new File(name + ".xml"));
    createXMLString(node, os);
  }

  public static String getNameWithoutExtension(String fileName) {
    int index = fileName.lastIndexOf('.');
    if (index!=-1) {
      return fileName.substring(0, index);
    }

    return "";
  }

  public static void createXMLString(Node document, OutputStream os) throws TransformerException {
    StringWriter writer = new StringWriter();
    TransformerFactory factory = TransformerFactory.newInstance();
    Transformer transformer = factory.newTransformer();

    transformer.setOutputProperty(OutputKeys.INDENT, "yes");
    transformer.setOutputProperty(OutputKeys.METHOD, "html");
    transformer.setOutputProperty("{http://xml.apache.org/xalan}indent-amount", "2");

    transformer.transform(new DOMSource(document), new StreamResult(os));
  }
}
