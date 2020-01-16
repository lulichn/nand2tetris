package net.lulichn.n2t.compiler;

import org.antlr.v4.runtime.CharStreams;
import org.antlr.v4.runtime.CommonTokenStream;
import org.antlr.v4.runtime.Parser;
import org.antlr.v4.runtime.tree.*;

import java.io.IOException;

public class Main {
  public static void main(String[] args) throws IOException {
    var lexer = new JackLexer(CharStreams.fromFileName(args[0]));
    var stream = new CommonTokenStream(lexer);
    var parser = new JackParser(stream);

    new P().visit(parser.klass());

    var walker = new ParseTreeWalker();
    var listener = new JackListenerImpl();
    walker.walk(listener, parser.klass());

    for (var s: listener.test()) {
      System.out.println(s);
    }
  }

  public static class P implements ParseTreeVisitor<String> {

    @Override
    public String visit(ParseTree tree) {
      return null;
    }

    @Override
    public String visitChildren(RuleNode node) {
      return null;
    }

    @Override
    public String visitTerminal(TerminalNode node) {
      return null;
    }

    @Override
    public String visitErrorNode(ErrorNode node) {
      return null;
    }
  }
}
