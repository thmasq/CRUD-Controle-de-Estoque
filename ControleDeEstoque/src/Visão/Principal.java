package Visão;

import java.awt.Font;
import java.awt.event.ActionEvent;
import java.awt.event.ActionListener;


import javax.swing.JButton;
import javax.swing.JFrame;
import javax.swing.JLabel;

/**

 *A classe Principal representa a classe principal do programa que controla o estoque.
 *Ela exibe uma janela com botões para cadastrar filiais de eletrônicos e móveis.
 *A classe possui uma janela (JFrame) onde os componentes visuais são adicionados.
 * @author Carlos Henrique
 * @author André João
 * @author Sunamita Vitória
 * 
 * @version 2.0
*/


public class Principal {
	
	
	JFrame janela = new JFrame("Controle de Estoque");
	JLabel titulo = new JLabel("Controle de Estoques");
	JButton Filial1 = new JButton("Cadastro Filial Eletronicos");
	JButton Filial2 = new JButton("Cadastro Filial Moveis");
	
	/**
	 * Construtor da classe `Principal`.
	 * Cria a janela, define as propriedades e adiciona os componentes.
	 */
	
	public Principal () {
		
	
		janela.setSize(394, 309);
		janela.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
		janela.setVisible(true);
		janela.getContentPane().setLayout(null);
		janela.setResizable(false);	
		janela.setLocationRelativeTo(null);
		
		titulo.setFont(new Font("Arial", Font.BOLD, 20));
		titulo.setBounds(85, 5, 210, 80);
		Filial1.setFont(new Font("Arial Black", Font.PLAIN, 11));
		
		Filial1.setBounds(74, 96, 221, 30);
		Filial2.setFont(new Font("Arial Black", Font.PLAIN, 11));
		Filial2.setBounds(74, 137, 221, 30);
		
		janela.getContentPane().add(titulo);
		janela.getContentPane().add(Filial1);
		janela.getContentPane().add(Filial2);
		
		
		// Botão que ativa a Visao de TelaEstoqueEletronico
		
		Filial1.addActionListener(new ActionListener() {
			
			public void actionPerformed(ActionEvent e) {
				TelaEstoqueEletronico telatstoquetletronico = new TelaEstoqueEletronico();
				telatstoquetletronico.setVisible(true);
			}
		});
		
		// Botão que ativa a Visao de TelaEstoqueMoveis
		
		Filial2.addActionListener(new ActionListener() {
			
			public void actionPerformed(ActionEvent e) {
				TelaEstoqueMoveis telaestoquemoveis = new TelaEstoqueMoveis();
				telaestoquemoveis.setVisible(true);
			}
		});
		
			
	}
	
	
	/**
	 * Método principal que inicia a execução do programa.
	 * Cria uma instância da classe `Principal`.
	 * 
	 * @param args argumentos de linha de comando (não utilizados)
	 */

	public static void main(String[] args) {
		new Principal();
		
	}

}
