
package Modelo;

	/**

	*Classe Eletronico que representa um item eletrônico.
	*Esta classe herda os atributos e métodos da classe Item.
	*Possui um atributo adicional para armazenar a voltagem do eletrônico.
	*Também possui um atributo para referenciar a filial associada ao eletrônico.
	*Possui construtores para inicializar os atributos e métodos de acesso e modificação para cada atributo.
	*Além disso, sobrescreve o método toString() para exibir informações específicas do eletrônico.
	
	* @author Carlos Henrique
	* @author André João
	* @author Sunamita Vitória
	* 
	* @version 2.0	
	*/

public class Eletronico extends Item {
	

	private Double voltagem;
	private Filial filial;

	/**
	 * Construtor da classe Eletronico com todos os parâmetros.
	 * 
	 * @param nomeitem            o nome do item eletrônico
	 * @param cor                 a cor do item eletrônico
	 * @param marca               a marca do item eletrônico
	 * @param modelo              o modelo do item eletrônico
	 * @param datadefabricação    a data de fabricação do item eletrônico
	 * @param valorproduto        o valor do item eletrônico
	 * @param codigoproduto       o código do item eletrônico
	 * @param quantidadeproduto   a quantidade do item eletrônico
	 * @param voltagem            a voltagem do item eletrônico
	 */
	public Eletronico(String nomeitem, String cor, String marca, String modelo, double datadefabricação,
	                  double valorproduto, double codigoproduto, double quantidadeproduto, Double voltagem) {
	    super(nomeitem, cor, marca, modelo, datadefabricação, valorproduto, codigoproduto, quantidadeproduto);
	    this.voltagem = voltagem;
	}

	/**
	 * Construtor da classe Eletronico apenas com a voltagem.
	 * 
	 * @param voltagem a voltagem do item eletrônico
	 */
	public Eletronico(Double voltagem) {
	    this.voltagem = voltagem;
	}

	/**
	 * Construtor vazio da classe Eletronico.
	 */
	public Eletronico() {
	}

	/**
	 * Método de acesso para obter a voltagem do item eletrônico.
	 * 
	 * @return a voltagem do item eletrônico
	 */
	public Double getVoltagem() {
	    return voltagem;
	}

	/**
	 * Método de modificação para definir a voltagem do item eletrônico.
	 * 
	 * @param voltagem a voltagem do item eletrônico
	 */
	public void setVoltagem(Double voltagem) {
	    this.voltagem = voltagem;
	}

	/**
	 * Método de acesso para obter a filial associada ao item eletrônico.
	 * 
	 * @return a filial associada ao item eletrônico
	 */
	public Filial getFilial() {
	    return filial;
	}

	/**
	 * Método de modificação para definir a filial associada ao item eletrônico.
	 * 
	 * @param filial a filial associada ao item eletrônico
	 */
	public void setFilial(Filial filial) {
	    this.filial = filial;
	}

	/**
	 * Sobrescrita do método toString() para exibir informações específicas do item eletrônico.
	 * 
	 * @return uma string com as informações do item eletrônico
	 */
	@Override
	public String toString() {
	    return "Produto: " + this.getNomeitem() + "\nMarca:" + this.getMarca() + "\nCor :" + this.getCor()
	            + "\nNome específico: " + this.getModelo() + "\nData De Fabricacao: " + this.getDatadefabricação()
	            + "\nCodigo:" + this.getCodigoproduto() + "\nquantidade: " + this.getQuantidadeproduto() + "\nvalor R$"
	            + this.getValorproduto();
	}
    
    
}
